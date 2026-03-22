* **Validation:** Checks if the audio data is long enough to be processed.
* **Windowing:** Applies a **Hann window** to the audio segments to reduce mathematical artifacts (spectral leakage) before analysis.
* **FFT Setup:** Prepares a **Fast Fourier Transform** (FFT) engine to convert audio from the time domain into the frequency domain.
* **Frame Processing:** * Iterates through the audio in small steps (`hop_size`).
    * Calculates the **magnitude** (volume) of each frequency bin for every frame.
    * Tracks the `global_peak` (the highest volume found in the entire signal).
* **Output:** Returns a `SpectrogramData` struct containing the frequency magnitudes and metadata.

---

### **Key Parameters**

| Parameter | Purpose |
| :--- | :--- |
| **`fft_size`** | Determines the frequency resolution (how many "notes" it detects). |
| **`hop_size`** | Determines the time resolution (how often it takes a "snapshot"). |
| **`frames_mag`** | The final matrix of frequency data. |


---
