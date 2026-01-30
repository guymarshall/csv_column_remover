<h2>csv_column_remover</h2>

Remove entire columns of ENORMOUS .csv files extremely efficiently without having to load the file into memory.

<h2>How do I run it?</h2>
You need Rust installed to compile and run this program. Please follow the instructions to install it from: <a href="https://www.rust-lang.org/tools/install">https://www.rust-lang.org/tools/install</a>
<br>
Once installed, you simply download this project, extract it into a directory, open a terminal, navigate to the directory, and run <code>cargo run --release -- --filename FILENAME</code> where FILENAME is the name of the .csv file you want to remove columns from.