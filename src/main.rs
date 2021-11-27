#![recursion_limit="1024"]

use anyhow::*;
use graphql_client::{GraphQLQuery, Response};
use prettytable::*;
use structopt::StructOpt;
use const_format::concatcp;

//=========================================
// Command Line Argument Parser (CLAP)
// Structured options setup 
// (Currently not impacting the program,
// these arguments can be ignored on the
// command line.)
//=========================================
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(required = true)]
    /// Enter part string such as '100pF 0603 16V'
    cap: String,

    #[structopt(short, long)]
    /// Enabled debug messages 
    debug_msg : bool, 

    #[structopt(short, long)]
    /// Filter High reliability (X7,X8,COG dielectrics & default filter)
    high_rel: bool,

    #[structopt(short, long)]
    /// Filter Low_Cost (Z5U,X5,X6 dielectrics)
    low_cost: bool,

    #[structopt(short, long)]
    /// Filter Nominal (X6,X7 dielectrics)
    nom: bool,
}

//=========================================
// Octopart API GraphQL Schema is in 
// "schema.graphql" while the desired
// search schema is in "query_1.graphql"
//=========================================
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.graphql",
    query_path = "./src/query_1.graphql",
    response_derives = "Debug,Serialize,PartialEq",
)]
struct CapQuery;

const  SEARCHES : i64 = 10; 
//===============================================
// Provide your own API Key here for Octopart
//===============================================
const API_KEY  : &str = "a9fe1e2b-e940-42f4-beef-704a0f3d667a";  // this is an example

const API_URL  : &str = "https://octopart.com/api/v4/endpoint?token=";
const ENDPOINT : &str =  concatcp!(API_URL, API_KEY);

//======================================================
// Main Entrypoint, CLAP argument called automatically
//======================================================
#[allow(non_snake_case)]
fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();
    let part_string = &opt.cap;
    let DEBUG_MSG = &opt.debug_msg; 

    let q = CapQuery::build_query(cap_query::Variables {
        cap_string: part_string.to_string(),
        total : SEARCHES,
    });

    let res = reqwest::blocking::Client::new()
        .post(ENDPOINT)
        .header("Content-Type", "application/json")
        .json(&q)
        .send()?;
 
    res.error_for_status_ref()?;
    let response_body: Response<cap_query::ResponseData> = res.json()?;
    let response_data: cap_query::ResponseData = response_body.data.expect("missing response data");
    if *DEBUG_MSG {
        println!("Response Data = {:?}", response_data);
    }
    

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b =>  "MPN", "MFG_Name", "Description", "Avail", "Octopart URL",));



    //====================================================
    // This shows current issue with Octopart GraphQL
    // requests in Rust - unable to get "price"
    // turn on DEBUG_MSG to see
    //====================================================
    if *DEBUG_MSG {
        for partx in response_data.search.results.iter(){
            for item in partx.iter(){
                let pricex = &item.part.median_price_1000;
                println!("PriceX = {:?}",pricex);
                if pricex.is_some(){
                    for xx in pricex.iter().next(){

                        println!("PriceY is = {:?}",*xx);
                    }
                    
                }
            }
        }
    }
    println!("***************************************");
    println!("**** Results for {}    ", &part_string);
    println!("***************************************");
    for mfg in response_data.search.results.iter(){
        for item in mfg.iter(){

            let mut index = 0; 
            if index == 0 {
                if item.part.avg_avail > 0.0 {
                
                if *DEBUG_MSG {
                    println!("{:?}{:?}{:?}",item.part.mpn, item.part.manufacturer.name, item.part.short_description);
                }
            
                table.add_row(row!(item.part.mpn, 
                    item.part.manufacturer.name, 
                    item.part.short_description, 
                    item.part.avg_avail,
                    item.part.octopart_url));
                }
            }
            let mut total_descrip = String::new() ;
            for text in item.part.descriptions.iter(){
                let descrip = format!("{:?}", text);
                if item.part.avg_avail > 0.0 {
                    total_descrip.push_str(&descrip);
                }
                index+=1; 
            }
        }
    }
    table.printstd();
    Ok(())
}


