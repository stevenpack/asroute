extern crate cymrust;
extern crate crossterm;

use std::io::{self, BufRead};
use cymrust::{AsNumber};

fn main() {
  //TODO: debug (show traceroute and failed lines)
  //TODO: pass through the ***
  //TODO: -t show traceroute output
  //TODO: -v show debug lines and -t
  //TODO: handle failures
  //TODO: show progress indicator
 
  let stdin = io::stdin();
  let mut last_asn = String::new();
  for line in stdin.lock().lines() {
    let line = line.expect("Could not read line from standard in");

    //For unknown * * * lines, continue.
    if line.contains("*") {
      println!("-> *");
      continue;
    }

    //Look for [ASXXX]. Error if it's not there
    let start_byte = line.find("[").unwrap_or(0);
    let end_byte = line.find("]").unwrap_or(0);

    if (start_byte == 0) || (end_byte == 0) {
      //if debug
      println!("Couldn't find [ASN] in line. Check you passed the -a argument to traceroute. Expected usage traceroute -a example.com | asroute");
      continue;
    }

    //Take the inside of the [ASXXXX]
    let asn = &line[start_byte + 1..end_byte];

    //Only show each new ASN
    if asn != last_asn {
      last_asn = asn.to_string();
      let num = asn.replace("AS", "");
      let lookup_asn: AsNumber = num.parse::<u32>().unwrap();
      
      //Don't look up AS0
      if lookup_asn == 0 {
        println!("-> AS0");
        continue;
      }

      //Lookup via WHOIS
      let cymru = cymrust::cymru_asn(lookup_asn);
      if cymru.is_ok() {
        //println!("{:#?}", cymru.unwrap());
        let vec = cymru.unwrap();
        let data = &vec[0];
        println!("-> {}", data.as_name);
      }  
    }    
  }

}