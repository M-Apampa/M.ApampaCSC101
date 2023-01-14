use std::io::Write;
use std::io;

fn code_7(){
    let name1 = "Name: Aigbona Juliet\nDept:Consulting\nQualification:B.Sc.\nServices:\n1.Analytics consulting services\n2.Customer experience\n3.Cybersecurity,strategy,risk,compliance and resilience\n4.Digital transformation\n5.Risk consulting services\n6.Supply chain and opeartors\n7.Technology transformation";

    let name2 ="Name: Akpevwe Iloka\nDept:Assurance\nQualification:HND\nServices:\n1.Audit services\n2.Climate change and sustainability services\n3.Financial accounting advisory services\n4.Forensic and integrity services\n5.Private client audit experience\n6.Accounting link\n7.Assurance";

    println!("Who would you like to create a file for:\n1.Aigbona Juliet\n2.Akpevwe Iloka");
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect("Not a valid number");
    let no:i32 = no.trim().parse().expect("Not a valid number");

    while no == 1{
        let mut file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
        file.write_all(name1.as_bytes()).expect("write failed");}


    while no == 2{
        let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
        file.write_all(name2.as_bytes()).expect("write failed");
    }
}
fn code_9(){
    let name3 = "Name:Ehis Ero\nDept:Strategy\nQualification:M.Sc.\nServices:\n1.Strategy consulting\n2.Corporate and growth strategy\n3.Transaction strategy and execution\n4.Restructuring and turnaround strategy\n5.Industry strategy\n6.Digital business building\n7.Commercial Strategy";

    let name4 = "Name: Maria Akinsola\nDept:Transaction and corporate finance\nQualification:M.Sc.\nServices:\n1.Corporate finance\n2.Divestments and carve-outs\n3.Sustainability and ESG Services\n4.M&A advisory\n5.M&A integration\n6.M&A technology and tools\n7.M&A advanced analytics";

     println!("Who would you like to create a file for:\n1.Ehis Ero\n2.Maria Akinsola");
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect("Not a valid number");
    let no:i32 = no.trim().parse().expect("Not a valid number");

    if no == 1{
         let mut file = std::fs::File::create("ehis_ero.txt").expect("create failed");
         file.write_all(name3.as_bytes()).expect("write failed");

    }

    if no == 2{
        let mut file = std::fs::File::create("maria_akinsola.txt").expect("create failed");
        file.write_all(name4.as_bytes()).expect("write failed");
    }

}
fn code_8(){
    let name5 = "Name: Adamu Sagamu\nDept:Tax\nQualification:B.Sc.\nServices:\n1.Tax planning\n2.Tax function operations\n3.Tax policy and controversy\n4.Global trade\n5.Tax accounting\n6.Tax compliance\n7.Transaction tax";

    let name6 = "Name: Gbenga Daniels\nDept:People and workforce\nQualification:HND\nServices:\n1.Change management and experience\n2.HR transformation\n3.Integrated workforce mobility\n4.Learning and development consulting\n5.Recognition and reward advisory\n6.Workforce analytics\n7.People and workforce";

     println!("Who would you like to create a file for:\n1.Adamu Sagamu\n2.Gbenga Daniels");
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect("Not a valid number");
    let no:i32 = no.trim().parse().expect("Not a valid number");

    if no == 1{
        let mut file = std::fs::File::create("adamu_sagamu.txt").expect("create failed");
        file.write_all(name5.as_bytes()).expect("write failed");

    }

    if no == 2{
         let mut file = std::fs::File::create("gbenga_daniels.txt").expect("create failed");
         file.write_all(name6.as_bytes()).expect("write failed");
    }

}

fn main() {
    let mut input = String::new();
    println!("Enter code:");
    io::stdin().read_line(&mut input).expect("Not a valid code");
    let code:i32 = input.trim().parse().expect("Not a valid code");

    if code == 7 {
        code_7()
    }
    else if code == 8{
        code_8()
    }
    else if code == 9{
        code_9()
    }
    else {
        println!("Not a valid code");
    }
 
}
