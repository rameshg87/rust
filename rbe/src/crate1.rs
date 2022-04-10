pub fn public_function() {
    println!("called rary's public function");
}

pub fn private_function() {
    println!("called rary's private function");
}

pub fn indirect_access_function() {
    println!("called rary's indirect access function");
    private_function();
}
