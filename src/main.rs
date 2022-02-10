extern crate regex;
use std::io;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
const WORD_LENGTH:usize = 5;
fn main() {
    let all_word = get_all_n_words(WORD_LENGTH);
    let mut answer_candidates = all_word.clone();
    let mut question_candidates = all_word.clone();
    let mut word = "tares".to_string();
    // let mut word = "crate".to_string();
    for _ in 0..6 {
        let hint = ask_hint(&word);
        answer_candidates   = reduct_answer_candidates(&answer_candidates, &word, &hint);
        // question_candidates = reduct_question_candidates(&question_candidates, &word, &hint);
        println!("{} - {}", answer_candidates.len(), question_candidates.len());
        if question_candidates.len() == 0 {
            question_candidates = answer_candidates.clone();
        }
        if answer_candidates.len() < 30 {
            println!("left");
            for i in answer_candidates.clone() {
                println!("\t{}", i);
            }
        }
        word = if answer_candidates.len() == 1 { 
            answer_candidates[0].clone()
        } else {
            guess_one_word(&answer_candidates, &question_candidates)
        };

    }
}
fn get_hint_map(qword:&String, answers: &Vec<String>) -> HashMap<String, u64>{
    let mut hint_map:HashMap<String, u64> = HashMap::<String, u64>::new();
    for ans_word in answers{
        let mut ans:String = ans_word.clone();
        for i in 0..WORD_LENGTH {
            let c = qword.chars().nth(i).unwrap();
            let a = ans.chars().nth(i).unwrap();
            if c == a {
                unsafe{
                    if let Some(elem) = ans.as_mut_vec().get_mut(i){
                        *elem = b'!';
                    }
                }
            }
        }
        let mut hint_str:String = String::new();
        for i in 0..WORD_LENGTH {
            let c = qword.chars().nth(i).unwrap();
            let a = ans.chars().nth(i).unwrap();
            if a == '!' {
                hint_str = hint_str + "!";
            } else if ans.contains(c) {
                hint_str = hint_str + "?";
            } else {
                hint_str = hint_str + "-";
            }
        }

        if !hint_map.contains_key(&hint_str) {
            hint_map.insert(hint_str.clone(), 1);
        }else if let Some(x) = hint_map.get_mut(&hint_str) {
            *x += 1;
        }
    }
    return hint_map;
}
fn guess_one_word(answers:&Vec<String>, questions:&Vec<String>) -> String {
    // answersに含まれる文字全体の内questionsに含まれる文字
    // ヒント全体は3^len 通り
    // これに対してanswers集合で, もっともヒントが多くなるquestionを探す
    let mut e_opt:f64 = 0.0;
    let mut q_best:String = "".to_string();
    let mut i:i32 = 0;
    for q in questions  {
        i += 1;
        let hint_map = get_hint_map(&q, &answers);
        let mut e:f64 = 0.0f64;
        for (_, val) in hint_map.iter(){
            let p:f64 = (*val as f64)/(answers.len() as f64);
            e += -p*p.log(2f64);
        }
        println!("({}/{}) {}:{}", i, questions.len(), q, e);
        if e >= e_opt {
            e_opt = e;
            q_best = q.to_string();
        }
    }

    let mut ea_opt:f64 = 0.0;
    let mut a_best:String = "".to_string();
    let mut i:i32 = 0;
    for q in answers {
        i += 1;
        let hint_map = get_hint_map(&q, &answers);
        let mut e:f64 = 0.0f64;
        for (_, val) in hint_map.iter(){
            let p:f64 = (*val as f64)/(answers.len() as f64);
            e += -p*p.log(2f64);
        }
        println!("({}/{}) {}:{}", i, questions.len(), q, e);
        if e >= ea_opt{
            ea_opt = e;
            a_best = q.to_string();
        }
    }

    if e_opt - ea_opt <= 1e-2 {
        return a_best;
    }else {
        return q_best;
    }
}
fn ask_hint(word:&String) -> String{
    println!("{}", word);
    let re = Regex::new(format!(r"^[!\?\-]{{{}}}", WORD_LENGTH).as_str()).unwrap();
    loop {
        let mut guess:String = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let hint = guess.trim().parse::<String>().unwrap();
        if re.is_match(&hint){
            return hint;
        // }else{
        }
    }
}

fn reduct_answer_candidates(candidate:&Vec<String>, word:&String, hint:&String)->Vec<String> {
    let mut novel_candidate:Vec<String> = candidate.clone();
    for i in 0..WORD_LENGTH {
        let mut tmp_candidate = vec![];
        let c = word.chars().nth(i).unwrap();
        let h = hint.chars().nth(i).unwrap();
        for w in novel_candidate {
            let wi = w.chars().nth(i).unwrap(); 
            if h == '-' && (!w.contains(c)){
                tmp_candidate.push(w.clone());
            } if h == '!' && wi == c {
                tmp_candidate.push(w.clone());
            }else if h == '?' && w.contains(c) && wi != c {
                tmp_candidate.push(w.clone());
            }
        }
        novel_candidate = tmp_candidate.clone();
    }
    return novel_candidate;
}

fn reduct_question_candidates(candidate:&Vec<String>, word:&String, hint:&String)->Vec<String> {
    let mut novel_candidate:Vec<String> = candidate.clone();
    for i in 0..WORD_LENGTH {
        let mut tmp_candidate = vec![];
        let c = word.chars().nth(i).unwrap();
        let h = hint.chars().nth(i).unwrap();
        for w in novel_candidate {
            let wi = w.chars().nth(i).unwrap();
            if h == '?' && c != wi {
                tmp_candidate.push(w.clone());
            } else if h == '-' && !w.contains(c) {
                tmp_candidate.push(w.clone());
            } else if h == '!'  {
                tmp_candidate.push(w.clone());
            }
        }
        println!("{}", tmp_candidate.len());
        novel_candidate = tmp_candidate.clone();
    }
    return novel_candidate;
}

fn get_all_n_words(n : usize)->Vec<String>{
    let fname = "/usr/share/dict/wordle-words.txt";
    // let fname = "/usr/share/dict/wordle-words-ans.txt";
    // let fname = "/usr/share/dict/qiita-words.txt";
    // let fname = "/usr/share/dict/words";
    let mut file = std::fs::File::open(fname).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut words : Vec<String> = vec![];
    let re_base = format!(r"^[a-z]{{{}}}$", n);
    let re = Regex::new(re_base.as_str()).unwrap();
    for line in contents.split('\n'){
        let s:String = line.trim().to_lowercase().to_string();
        if re.is_match(&s) {
            words.push(s);
        }
    }
    return words
}

