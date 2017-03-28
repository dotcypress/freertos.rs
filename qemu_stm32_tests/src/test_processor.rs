use super::*;
use prelude::v1::*;

use freertos_rs::*;
use freertos_rs::patterns::processor::*;

#[no_mangle]
pub extern fn test_processor() -> i8 {
    
    let main_task = Task::new().name("main").start(|| {

        let processor: Processor<Message<usize>> = Processor::new(5).unwrap();
        let client_1 = processor.new_client().unwrap();
        let client_2 = processor.new_client_with_reply(1, Duration::ms(100)).unwrap();

        let processor_task = Task::new().name("processor").start(move || {

            loop {
                if let Ok(msg) = processor.get_receive_queue().receive(Duration::ms(10)) {
                    debug_print(&format!("Received val {}", msg.get_val()));
                    let processed_message = msg.get_val() + 1;
                    processor.reply_val(msg, processed_message, Duration::ms(10)).expect("Failed to send the reply");
                    debug_print("Processed.");
                }
            }

        }).unwrap();

        {            
            client_1.send_val(1, Duration::ms(100));
        }
        
        {            
            let processed = client_2.call_val(2, Duration::ms(100)).expect("Missing the reply from the processor");
            assert_eq!(3, processed);
        }

        exit_test(0);

    }).unwrap();



	start_kernel();
	
	1
}