use reqwest::blocking::get;
use crate::settings::Domain;

pub fn make_update_request(url : &String) -> Result<(), Box<dyn std::error::Error>> {
    let _resp = get(url)?;
    Ok(())
}



pub fn make_updates(domains: &Vec<Domain>, url: &String) {
    
    for domain in domains {
        let mut uri : String = url.clone();
        uri = uri.replace("<domain>", &domain.domain);
        uri = uri.replace("<pass>", &domain.pass.to_string());
        //println!("{:?}", uri);
        match make_update_request(&uri){
            Ok(_) => println!("FreeDynDnss: Update Request for {} successful", domain.domain),
            Err(_) => println!("FreeDynDnss: Update Request for {} failed", domain.domain),
        }


    }


}