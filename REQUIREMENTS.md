# Market UDP Data Parser

## Overview

A high-performance Rust application that parses and processes market data quotes from PCAP files containing UDP packets. The application focuses on efficient memory usage and concurrent processing to handle files larger than available memory.

## Requirements

### Core Functionality

1. **Parse PCAP files** containing UDP market data packets
2. **Extract quote messages** starting with ASCII bytes "B6034"
3. **Print formatted output** with packet times, quote data, and bid/ask information
4. **Optional reordering** by quote accept time using the `-r` flag

### Performance Constraints

- **Memory Efficiency**: Must handle input files larger than available memory
- **Speed Optimization**: Optimized for both wall time and memory usage
- **Rust Version**: Compatible with latest stable rustc release

### Command Line Interface

```bash
# Basic parsing
./parse-quote mdf-kospi200.20110216-0.pcap

# With reordering by quote accept time
./parse-quote -r mdf-kospi200.20110216-0.pcap
```

### Output Format

```
<pkt-time> <accept-time> <issue-code> <bqty5>@<bprice5> ... <bqty1>@<bprice1> <aqty1>@<aprice1> ... <aqty5>@<aprice5>
```

**Field Order:**
- Packet timestamp from PCAP
- Quote accept time from packet
- Issue code (ISIN)
- Bids: 5th → 1st (quantity@price)
- Asks: 1st → 5th (quantity@price)

## Packet Specification

### Quote Packet Structure (B6034)

| Field | Length | Description | Notes |
|-------|--------|-------------|-------|
| Data Type | 2 | B6 | Fixed |
| Information Type | 2 | 03 | Fixed |
| Market Type | 1 | 4 | Fixed |
| Issue Code | 12 | ISIN code | Identifier |
| Issue Seq No. | 3 | Sequence number | |
| Market Status Type | 2 | Status | |
| Total Bid Quote Volume | 7 | Total bid volume | |
| Best Bid Price (1st-5th) | 5 each | Price levels | Decimals |
| Best Bid Quantity (1st-5th) | 7 each | Quantities | Decimals |
| Total Ask Quote Volume | 7 | Total ask volume | |
| Best Ask Price (1st-5th) | 5 each | Price levels | |
| Best Ask Quantity (1st-5th) | 7 each | Quantities | |
| Bid Valid Quote Counts | 5+4×5 | Quote counts | |
| Ask Valid Quote Counts | 5+4×5 | Quote counts | |
| **Quote Accept Time** | 8 | HHMMSSuu format | **Key for reordering** |
| End of Message | 1 | 0xff | Terminator |

### Filtering Rules

- **Include**: Packets starting with "B6034"
- **Ignore**: All other UDP packets
- **Ports**: Original data from 15515/15516 (informational)

## Reordering Requirements

### Time-Based Ordering (`-r` flag)

- **Sort by**: Quote accept time field (HHMMSSuu format)
- **Window Constraint**: Packet time vs accept time difference ≤ 3 seconds
- **Out-of-order Handling**: Buffer and reorder packets within the time window

### Concurrency Considerations

- **Async I/O**: Use Tokio for file reading and processing
- **Bounded Channels**: Implement backpressure for memory management  
- **Sliding Window**: Time-based buffer for packet reordering
- **Shared State**: Thread-safe counters and statistics

## Technical Architecture

### Core Components

1. **PCAP Parser**: Extract UDP packets from PCAP format
2. **Quote Decoder**: Parse B6034 packet structure
3. **Time Window Manager**: Handle reordering with 3-second constraint
4. **Output Formatter**: Generate specified output format
5. **Memory Manager**: Handle large files efficiently

### Performance Features

- **Memory Mapping**: For large file access
- **Streaming Processing**: Avoid loading entire file
- **Concurrent Pipeline**: Parallel packet processing stages
- **Zero-Copy**: Minimize data copying where possible

## Test Data

- Sample file: Kospi 200 market data from 2011-02-16
- Duration: First 30 seconds of trading
- Source: Tsuru Capital (© 2010-2024)

## Success Criteria

1. **Functional**: Correctly parse and format all B6034 packets
2. **Performance**: Handle files larger than available RAM
3. **Correctness**: Accurate reordering within 3-second window
4. **Efficiency**: Optimal memory and CPU usage
5. **Compliance**: Compile with stable Rust toolchain