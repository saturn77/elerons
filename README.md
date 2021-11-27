# Elerons - ELectRONics Search with Rust 
Elerons is a command line tool for electronic component selection.  

## Background

There are several Octopart interfaces available, many of them in Python or Javascript. Elerons takes a different approach and employs Rust. The graphql-client crate is used along with reqwest to perform the tasks at hand. The motivation for this approach is the speed, along with the ability to scale the program in the future with async requests. 

Elerons is designed to be a fast, high perforant search for components on Octopart via the command line. 

For example, on Linux

```Terminal
./elerons "330pF 0603 16V" 
```
While on Windows

```Terminal
elerons.exe "100pF 0402 10V" 
```

Electronic component searches are often involved and tedious. The real objective of the hardware engineer is to find a suitable part, and Elerons has filters built in such as "high reliability" and "nominal" and "low cost". (These filters are works in progress.) Elerons will not only does the search, but will apply filters that are based on engineering know how. The usage from the command line allows for individual searches which is often quite useful when developing hardware, but can easily be extended to search an entire BOM for parts. 

## Using 

An example of running a search from the command line is shown below, showing the immediate response of the part request query. 

<img src="media/Elerons.gif">

## Rust 
The requests are made using the graphql-client and reqwest crates. There is some effort to efficiently handle the grapql responses. 
```Rust 
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
```
