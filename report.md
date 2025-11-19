# Job Description
- Muhammad Rasya Alghifari - 33.33%: Perform analysis, testing, and write the final report.
- Muhammad Ilham Akbar - 33.33%: Handled input, CLI interface, and testing.
- Alfarel Sandriano Subektiansyah - 33.33%: Core image compression logic and algorithm

# Huffman Image Compression Report

This project implements a lossless image compression system using Huffman coding combined with delta encoding. 
## Algorithm Analysis

### 1. Huffman Coding

Huffman coding is a compression technique that assigns shorter codes to frequently occurring values and longer codes to rare values. This is similar to how Morse code uses shorter patterns for common letters like "E" (.) compared to less common letters like "Q" (--.-).

#### Step 1: Frequency Analysis

The algorithm first counts how often each byte value appears in the image data:

```rust
fn build_tree(&mut self, data: &[u8]) {
    let mut freq_map: HashMap<u8, usize> = HashMap::new();
    
    // Count how many times each byte appears
    for &byte in data {
        *freq_map.entry(byte).or_insert(0) += 1;
    }
}
```

**Example:** If we have image data `[100, 100, 100, 50, 50, 200]`:
- Byte 100 appears 3 times (50% frequency)
- Byte 50 appears 2 times (33% frequency)  
- Byte 200 appears 1 time (17% frequency)

#### Step 2: Building the Huffman Tree

The algorithm creates a binary tree by repeatedly merging the two nodes with the lowest frequencies:

```rust
// Create a priority queue (min-heap) with leaf nodes
let mut heap = BinaryHeap::new();
for (val, freq) in freq_entries {
    heap.push(HeapEntry {
        node: Node::new_leaf(freq, val),
    });
}

// Merge nodes until only one remains (the root)
while heap.len() > 1 {
    let left = heap.pop().unwrap();   // Get lowest frequency
    let right = heap.pop().unwrap();  // Get second lowest
    let freq = left.node.freq + right.node.freq;
    heap.push(HeapEntry {
        node: Node::new_internal(freq, left.node, right.node),
    });
}
```

#### Step 3: Generating Binary Codes

The tree is traversed to assign binary codes: left branches = 0, right branches = 1.

```rust
fn generate_codes_helper(
    node: &Node,
    code: &mut Vec<bool>,
    codes: &mut HashMap<u8, Vec<bool>>,
) {
    if let Some(val) = node.value {
        // Leaf node - save the code for this byte
        codes.insert(val, code.clone());
    } else {
        // Internal node - traverse children
        if let Some(ref left) = node.left {
            code.push(false);  // Add 0 for left
            Self::generate_codes_helper(left, code, codes);
            code.pop();
        }
        if let Some(ref right) = node.right {
            code.push(true);   // Add 1 for right
            Self::generate_codes_helper(right, code, codes);
            code.pop();
        }
    }
}
```

**From our example tree:**
- Byte 100 → `0` (most frequent, shortest code)
- Byte 50 → `10` (less frequent, longer code)
- Byte 200 → `11` (least frequent, longer code)

#### Step 4: Encoding Data

Replace each byte with its Huffman code:

```rust
fn encode(&self, data: &[u8]) -> Vec<bool> {
    let mut res = Vec::new();
    for &byte in data {
        if let Some(code) = self.codes.get(&byte) {
            res.extend_from_slice(code);  // Append the code bits
        }
    }
    res
}
```

**Example encoding `[100, 100, 100, 50, 50, 200]`:**
- Original: 6 bytes × 8 bits = 48 bits
- Encoded: `0 + 0 + 0 + 10 + 10 + 11` = 8 bits 

#### Step 5: Decoding Data

Traverse the tree following each bit until reaching a leaf node:

```rust
fn decode(&self, bits: &[bool]) -> Vec<u8> {
    let mut res = Vec::new();
    if let Some(ref tree) = self.tree {
        let mut curr = tree;
        for &bit in bits {
            // Navigate: false=left, true=right
            curr = if bit {
                curr.right.as_ref().unwrap()
            } else {
                curr.left.as_ref().unwrap()
            };
            
            if let Some(val) = curr.value {
                res.push(val);
                curr = tree;
            }
        }
    }
    res
}
```

**Decoding `00010011`:**
1. Read `0` → go left → found byte 100
2. Read `0` → go left → found byte 100
3. Read `0` → go left → found byte 100
4. Read `10` → go right, then left → found byte 50
5. Read `10` → go right, then left → found byte 50
6. Read `11` → go right, then right → found byte 200

Result: `[100, 100, 100, 50, 50, 200]` 

### 2. Delta Encoding

Delta encoding transforms data by storing differences between consecutive values instead of absolute values. This effective for images where neighboring pixels often have similar colors.

#### Applying Delta Encoding

```rust
fn apply_delta_encoding(&self, data: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(data.len());
    res.push(data[0]);  // First value stays unchanged
    
    // Store differences between consecutive bytes
    for i in 1..data.len() {
        res.push(data[i].wrapping_sub(data[i - 1]));
    }
    
    res
}
```

**Example with a gradient: `[100, 102, 105, 103, 101, 100]`**

Original values:
```
[100, 102, 105, 103, 101, 100]
```

After delta encoding:
```
[100, 2, 3, -2, -2, -1]  (represented as [100, 2, 3, 254, 254, 255] in u8)
```


#### Reversing Delta Encoding

During decompression, we reconstruct the original by accumulating the differences:

```rust
fn reverse_delta_encoding(&self, data: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(data.len());
    res.push(data[0]);  // First value unchanged
    
    // Reconstruct by adding each difference to the previous value
    for i in 1..data.len() {
        res.push(res[i - 1].wrapping_add(data[i]));
    }
    
    res
}
```

**Reversing `[100, 2, 3, 254, 254, 255]`:**
1. Start with 100
2. 100 + 2 = 102
3. 102 + 3 = 105
4. 105 + 254 = 103 (wraps around)
5. 103 + 254 = 101 (wraps around)
6. 101 + 255 = 100 (wraps around)

Result: `[100, 102, 105, 103, 101, 100]` 


## Program Structure

```
huffman-image-compression/
├── src/
│   ├── main.rs                     # Entry point
│   ├── lib.rs                      # Library root
│   ├── input_handler/              # CLI and input validation
│   │   ├── mod.rs
│   │   ├── cli.rs                  # Command-line interface
│   │   └── ensure_valid_extension.rs
│   └── logic/                      # Core compression logic
│       ├── mod.rs
│       ├── image_compressor.rs     # Main compression/decompression
│       ├── node.rs                 # Huffman tree node
│       ├── image_metadata.rs       # Image metadata structure
│       └── color_type.rs           # Color type enum
├── Cargo.toml                      # Dependencies
└── example/                        # Test images
```


## Program Output

### Compression 

```zsh
❯ cargo run -- compress example/img_file/sample.bmp  example/himg_file/himage_sample.himg
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/huffman-image-compression compress example/img_file/sample.bmp example/himg_file/himage_sample.himg`
```
### Decompression 

```zsh
❯ cargo run -- decompress example/himg_file/himage_sample.himg example/img_file/compressed.jpg
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/huffman-image-compression decompress example/himg_file/himage_sample.himg example/img_file/compressed.jpg`
```

### File Size Comparison

```zsh
❯ ls -la example/img_file/
.rw-r--r-- 758k rasya rasya 2025-11-19 19:00 -N compressed.jpg
.rw-r--r-- 7.4M rasya rasya 2025-11-19 18:56 -N sample.bmp
```
in this example it achives about 90% size reduction.



### Compression 

```zsh
❯ cargo run -- compress example/img_file/sample.bmp  example/himg_file/himage_sample.himg
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/huffman-image-compression compress example/img_file/sample.bmp example/himg_file/himage_sample.himg`
```
### Decompression 

```zsh
❯ cargo run -- decompress example/himg_file/himage_sample.himg example/img_file/compressed.jpg
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/huffman-image-compression decompress example/himg_file/himage_sample.himg example/img_file/compressed.jpg`
```

### File Size Comparison

```zsh
❯ ls -la example/img_file/
.rw-r--r-- 758k rasya rasya 2025-11-19 19:00 -N compressed.jpg
.rw-r--r-- 7.4M rasya rasya 2025-11-19 18:56 -N sample.bmp
```
in this example it achieves about 90% size reduction.

### Image Quality Comparison
<table>
<tr>
<td><img src="example/img_file/sample.bmp" width="100%"/></td>
<td><img src="example/img_file/compressed.jpg" width="100%"/></td>
</tr>
<tr>
<td align="center">Before</td>
<td align="center">After</td>
</tr>
</table>
It might be really hard to see at this scale, you might want to use [online image comparison tool](https://www.diffchecker.com/image-compare/) to see the differences. if we look closely, the compressed image is visibly noisier.