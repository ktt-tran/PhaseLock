pub mod decode; /* symphonia crate used to decodes message in time domain for a variety of audio file types */
pub mod resample; /* make message into the same sample rate */
pub mod process; /* process reference and unknown message */
pub mod fft; /* transform message into frequency domain for voice recogintion */
pub mod mae; /* mean absolute error between two signals */
pub mod similarity; /* cosine similarity for matching the content of the message */