use file_encrypt::newfile;
use file_encrypt::cbc;

fn main() {
    newfile::output_file();
    cbc::main();
}
