 // Simple bump allocator for memory management                                                                                                                                                                                 
                                                                                                                    
extern "C" {
    static _end: usize;
}         

static mut PLACEMENT_PTR: Option<usize> = None;                                                                    
                                                                                                                    
// Initialize the allocator with _end pointer                                                                                                                              
pub unsafe fn init_alloc() {                                                                                       
    PLACEMENT_PTR = Some((&_end) as *const usize as usize);                                                        
} 

// Allocate memory                                                                                                                  
 pub unsafe fn alloc(size: usize) -> *mut u8 {
    let address = PLACEMENT_PTR.unwrap();                                                                          
    PLACEMENT_PTR = Some(address + size);                                                                          
    address as *mut u8                                                                                             
 } 