use std::io;

fn geopo_merger() {
    let noc:[&str;6] = ["NAME OF COMMISSIONER","Aigbogun Alamba Daudu","Murtala Afeez Bendu",
    "Okorocho Calistus Ogbona","Adewale  Jimoh Akanbi","Osazuwa Faith Etieye"];

    let ministry:[&str;6] = ["MINISTRY","Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];

    let geozo:[&str;6] = ["GEOPOLITICAL ZONE","South West","North East","South South","South West","South East"];
    for index in 0..6{
        println!("{}       {}       {}       {}",index,noc[index],ministry[index],geozo[index] );
    }

}

fn pub_service(){
    let ps:[i32;6] = [1-2,3-5,5-8,8-10,10-13,14];

    let oa:[&str;6] = ["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];

    let a:[&str;6] = ["-","Research Assistant","PhD Candidate","Post-Doc researcher","Senior Lecturer","Dean"];

    let l:[&str;6] = ["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];

    let t:[&str;6] = ["Placement","Classroom Teacher","Snr Teacher","Leading teacher","Deputy Principal","Principal"];

    let mut job = String::new();
    println!("Are you a/an\n1.Office Administrator\n2.Academic\n3.Lawyer\n4.Teacher");
    io::stdin().read_line(&mut job).expect("Not a valid string");
    let job:i32 = job.trim().parse().expect("Not a valid answer");

    let mut experience = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    let experience:i32 = experience.trim().parse().expect("Not a valid answer");  

    for index in 0..6{
    if job == 1 && experience <= 1 && experience >=2{
        println!("You have been promoted to a/an {}", oa[0]);
    }
    else if job == 1 && experience <= 3 && experience >=5{
        println!("You have been promoted to a/an {}", oa[1]);
    }
    else if job == 1 && experience <= 5 && experience >=8{
        println!("You have been promoted to a/an {}", oa[2]);
    }
    else if job == 1 && experience <= 8 && experience >=10{
        println!("You have been promoted to a/an {}", oa[3]);
    }
    else if job == 1 && experience <= 10 && experience >=14{
        println!("You have been promoted to a/an {}", oa[4]);
    }
    else if job == 1 && experience <= 14{
        println!("You have been promoted to a/an {}", oa[5]);
    }

    if job == 2 && experience <= 1 && experience >=2{
        println!("You have been promoted to a/an {}", a[0]);
    }
    else if job == 2 && experience <= 3 && experience >=5{
        println!("You have been promoted to a/an {}" ,a[1]);
    }
    else if job == 2 && experience <= 5 && experience >=8{
        println!("You have been promoted to a/an {}" ,a[2]);
    }
    else if job == 2 && experience <= 8 && experience >=10{
        println!("You have been promoted to a/an {}" ,a[3]);
    }
    else if job == 2 && experience <= 10 && experience >=14{
        println!("You have been promoted to a/an {}" ,a[4]);
    }
    else if job == 2 && experience <= 14{
        println!("You have been promoted to a/an {}" ,a[5]);
    }

    if job == 3 && experience <= 1 && experience >=2{
        println!("You have been promoted to a/an {}", l[0]);
    }
    else if job == 3 && experience <= 3 && experience >=5{
        println!("You have been promoted to a/an {}" ,l[1]);
    }
    else if job == 3 && experience <= 5 && experience >=8{
        println!("You have been promoted to a/an {}" ,l[2]);
    }
    else if job == 3 && experience <= 8 && experience >=10{
        println!("You have been promoted to a/an {}" ,l[3]);
    }
    else if job == 3 && experience <= 10 && experience >=14{
        println!("You have been promoted to a/an {}" ,l[4]);
    }
    else if job == 3 && experience <= 14{
        println!("You have been promoted to a/an {}" ,l[5]);
    }
    if job == 4 && experience <= 1 && experience >=2{
        println!("You have been promoted to a/an {}",t[0]);
    }
    else if job == 4 && experience <= 3 && experience >=5{
        println!("You have been promoted to a/an {}",t[1]);
    }
    else if job == 4 && experience <= 5 && experience >=8{
        println!("You have been promoted to a/an {}" ,t[2]);
    }
    else if job == 4 && experience <= 8 && experience >=10{
        println!("You have been promoted to a/an {}",t[3]);
    }
    else if job == 4 && experience <= 10 && experience >=14{
        println!("You have been promoted to a/an {}" ,t[4]);
    }
    else if job == 4 && experience <= 14{
        println!("You have been promoted to a/an {}",t[5]);
    }
    }

}

fn main(){
    let mut division = String::new();
    println!("Which division would you like to access:\n1.Geopolitical Zoning \n2.Pulic Service Checker");
    io::stdin().read_line(&mut division).expect("Not a valid string");
    let division:i32 = division.trim().parse().expect("Not a valid answer");

    if division == 1{
        println!("Geopolitical Zoning System");
        geopo_merger()
    }
    if division == 2{
        println!("Staff Employment Position Level System");
        pub_service()
    }
}

