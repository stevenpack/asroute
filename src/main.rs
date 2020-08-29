extern crate cymrust;

use std::io::{self, BufRead};
use std::process;
use cymrust::{AsNumber};

fn main() {
  //TODO: -h with usage
  //TODO: -t show traceroute output
  //TODO: -v show debug lines and -t
  //TODO: show progress indicator
  //TODO: brew install https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
  //TODO: README

  let mut last_asn_str = String::new();

  let stdin = io::stdin();
  for line in stdin.lock().lines() {    
    //Read a line
    let line = match line {
      Ok(line) => line.to_uppercase(),
      Err(e) => {
        eprintln!("Failed to read line. {}", e);
        process::exit(1);
      } 
    };

    //For unknown * * * lines, continue.
    if line.contains("*") {
      println!("-> *");
      continue;
    }

    //Don't look up AS0
    if line.contains("[AS0]") {
      println!("-> AS0");
      continue;
    } 

    //Look for [ASXXX]. Error if it's not there
    let start_index  = line.find("[").unwrap_or(usize::MAX);
    let end_index = line.find("]").unwrap_or(usize::MAX);

    if (start_index == usize::MAX) || (end_index == usize::MAX) {
      eprintln!("Couldn't find [ASN] in line. Check you passed the -a argument to traceroute. Expected usage traceroute -a example.com | asroute");
      continue;
    }

    //Take the inside of the [ASXXXX]
    let asn_str = &line[start_index + 1..end_index];
    
    //Only show ASN when it changes
    if asn_str != last_asn_str {
      last_asn_str = asn_str.to_string();
      let num_str = asn_str.replace("AS", "");
      let asn: AsNumber = match num_str.parse::<u32>() {
        Ok(val) => val,
        Err(e) => {
          eprintln!("Failed to parse ASN. {}", e);
          continue;
        }
      };
      
      //Lookup via WHOIS
      let asn_info = match cymrust::cymru_asn(asn){
        Ok(val) => val,
        Err(e) => {
          eprintln!("Failed to lookup ASN {}, {}", asn, e);
          continue;
        }
      };
      let mut as_name = "?";
      if asn_info.len() > 0 {
        let data = &asn_info[0];
        as_name = &data.as_name;
      }
      println!("-> {}", as_name);
    }    
  }
}