use std::{fs::File, io::BufReader, thread, time::Duration};
use clap::Parser;
use notify_rust::{Notification, Timeout};
use rodio::{source::SineWave, Decoder, OutputStream, OutputStreamHandle, Sink, Source};
#[derive(Parser,Debug)]
#[command(version,about,long_about= None)]
struct Args{
    #[arg(short='i',long)]
    interval:Option<u32>,
    #[arg(short='a',long)]
    audio:Option<String>,
    #[arg(short='o',long)]
    once:bool
}
fn main() {
    //interval handling and checking if the user has provided an audio file path
    let args=Args::parse();
    let(interval,audio)=match (args.interval,args.audio){
        (Some(interval),Some(audio))=> (interval,audio),
        (None,Some(audio))=> (30,audio),
        (Some(interval),None)=> (interval,"".to_string()),
        (None,None)=> (30,"".to_string())
    };
    //notify user the timer has started
    Notification::new()
    .summary("hydr-rmd Activated")
    .body(&format!("I will keep popping up every {} minutes  to remind you to drink water",interval))
    .icon("water")
    .timeout(Timeout::Milliseconds(5900000))
    .show().unwrap()
    .wait_for_action(|action| match  action {
        "Default action"=>println!("Default action triggered"),
        "clicked"=>println!("Clicker action triggered"),
        _=>println!("Unknown action triggered")
    });
 //main-loop
    loop{
        if args.once{
            thread::sleep(Duration::from_secs((interval  * 60u32).into()));
            notify(audio.as_str());
            break;
        }else{
            thread::sleep(Duration::from_secs((interval  * 60u32).into()));
            notify(audio.as_str());
        }
}}
fn notify(audio:&str){
    let notify_thread=thread::spawn(||{

        Notification::new()
        .summary("hydr-rmd")
        .body("Time to drink water")
        .icon("water")
        .timeout(Timeout::Milliseconds(5900000))
        .show().unwrap()
        .wait_for_action(|action| match  action {
            
        
            "Default action"=>println!("Default action triggered"),
            "clicked"=>println!("Clicker action triggered"),
            _=>println!("Unknown action triggered")
        })
        ;
    
        });
        let audio_path=audio.to_string();
        let audio_thread=thread::spawn(move ||{
    
        //audio handling 
       if  let Ok((_stream,stream_handle))=OutputStream::try_default(){
        let sink=Sink::try_new(&stream_handle).unwrap();
        let beep=SineWave::new(880).take_duration(Duration::from_secs_f32(5.00)).amplify(0.20);
        if let Ok(file) = File::open(&audio_path) {
            let file = BufReader::new(file); 
            if let Ok(decoder) = Decoder::new(file) {
                sink.append(decoder);
            } else {
                eprintln!("Failed to decode audio file, using beep...");
                sink.append(beep); 
            }
        } else {
            sink.append(beep);
        }
       
        sink.sleep_until_end();
       }
       
        });
        audio_thread.join().unwrap();
        notify_thread.join().unwrap();
}

