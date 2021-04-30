use crate::parse::idx_after;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub(crate) struct Counter {
    pub(crate) packets: u64,
    pub(crate) bytes: u64,
}

impl Counter {
    // ... [3907:262379]
    //      ^    ^ Bytes
    //      ^ Packets
    pub(crate) fn parse_counter<S: AsRef<str>>(line: S) -> Self {
        let line = line.as_ref();
        let len = line.len();
        let packet_start = idx_after(0, line, '[').unwrap_or(0);
        let packet_end = idx_after(packet_start + 1, line, ':').unwrap_or(len);

        let bytes_start = packet_end + 1;
        let bytes_end = idx_after(bytes_start, line, ']').unwrap_or(len);

        Self {
            packets: line[packet_start + 1..packet_end].parse().unwrap_or(0),
            bytes: line[bytes_start..bytes_end].parse().unwrap_or(0),
        }
    }

    #[inline]
    pub(crate) fn bytes(&self) -> u64 {
        self.bytes
    }

    #[inline]
    pub(crate) fn packets(&self) -> u64 {
        self.packets
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counter_parse_base() {
        assert_eq!(
            Counter::parse_counter("... [3907:262379]"),
            Counter {
                packets: 3907,
                bytes: 262379
            }
        );
    }

    #[test]
    fn counter_parse_rule_0() {
        assert_eq!(
            Counter::parse_counter(
                "[0:0] -A LIBVIRT_OUT -o virbr0 -p tcp -m tcp --dport 68 -j ACCEPT"
            ),
            Counter {
                packets: 0,
                bytes: 0,
            }
        );
    }

    #[test]
    fn counter_parse_rule() {
        assert_eq!(
            Counter::parse_counter("[607533:364542212] -A OUTPUT -j LIBVIRT_OUT"),
            Counter {
                packets: 607533,
                bytes: 364542212,
            }
        );
    }

    #[test]
    fn counter_parse_chain_0() {
        assert_eq!(
            Counter::parse_counter(":LIBVIRT_OUT - [0:0]"),
            Counter {
                packets: 0,
                bytes: 0,
            }
        );
    }

    #[test]
    fn counter_parse_chain() {
        assert_eq!(
            Counter::parse_counter(":FORWARD ACCEPT [3:156]"),
            Counter {
                packets: 3,
                bytes: 156,
            }
        );
    }
}
