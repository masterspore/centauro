use std::fs;

/*
The logger will use the ELF system. It will run in a separate thread by using a mpsc
channel, in which other threads will add logs, and this thread will write into the appropiate file.
*/