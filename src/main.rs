use regex::Regex;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {

    // yyyy-mm-dd hh:mm:ss 패턴 정규식
    let date_time_pattern = r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}";

    // 정규식 생성
    let re = Regex::new(date_time_pattern).unwrap();

    // 테스트를 위한 문자열 생성
    // 진행률 바 생성
    let total_steps = 100_000_000;
    let pb = ProgressBar::new(total_steps);
     // 진행률 바 스타일 설정
     pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    println!("CREATE STRINGS...");
    let mut test_strings = Vec::new();
    for i in 0..total_steps {
        if i % 2 == 0 {
            test_strings.push(format!("Event at 2024-12-26 15:{:02}:45", i % 60));
        } else {
            test_strings.push(format!("Random text with no date {}", i));
        }
    }

   // 정규식 테스트 시작
   println!("START!");
   let start_time = Instant::now();

   let mut match_count = 0;
   let mut count = 0;
   for text in &test_strings {
        if re.is_match(text) {
            match_count += 1;
        }

       
        pb.inc(1);
       

        count = count + 1;
   }

   let duration = start_time.elapsed();
   println!("DONE!");
   pb.finish_with_message("Done!");

   // 결과 출력
   println!("Checked {} strings.", test_strings.len());
   println!("Found {} matches.", match_count);
   println!("Time taken: {:?}", duration);
   
}