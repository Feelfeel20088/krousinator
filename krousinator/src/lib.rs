// use tokio::{fs::File, io::AsyncWriteExt};

// #[no_mangle]
// pub extern "C" fn init() {
//     let rt = tokio::runtime::Runtime::new().unwrap();

//     rt.block_on(async {
//         let mut output = File::create("/home/felix/projects/krousinator/krousinator/src/output.log")
//             .await
//             .unwrap();

//         loop {
//             output.write_all(b"Library injected successfully!\n").await.unwrap();

//             // Add a small delay to avoid spamming the disk
//             println!("hello my freind the so was injected and working");
//             tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//         }
//     });
// }

#[no_mangle]
pub extern "C" fn init() {
    loop {
        let _ = std::fs::write("/tmp/injected.txt", b"Hello from injected .so!\n");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
