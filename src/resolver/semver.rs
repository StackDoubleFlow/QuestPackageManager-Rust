use std::fmt;

use semver::{Comparator, Op, Prerelease, VersionReq};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(semver::Version);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VersionSet(VersionReq);

impl pubgrub::version::Version for Version {
    fn lowest() -> Self {
        Self(semver::Version::new(0, 0, 1))
    }

    fn bump(&self) -> Self {
        let mut v = self.0.clone();
        v.patch += 1;
        Self(v)
    }
}

const EMPTY_SET: &[Comparator] = &[Comparator {
    op: Op::Less,
    major: 0,
    minor: Some(0),
    patch: Some(1),
    pre: Prerelease::EMPTY,
}];

impl pubgrub::version_set::VersionSet for VersionSet {
    type V = Version;

    fn empty() -> Self {
        Self(VersionReq {
            comparators: EMPTY_SET.to_vec(),
        })
    }

    fn singleton(v: Self::V) -> Self {
        let Version(semver::Version {
            major,
            minor,
            patch,
            pre,
            ..
        }) = v;

        Self(VersionReq {
            comparators: vec![Comparator {
                op: Op::Exact,
                major,
                minor: Some(minor),
                patch: Some(patch),
                pre,
            }],
        })
    }

    fn complement(&self) -> Self {
        let mut comparators = Vec::with_capacity(self.0.comparators.len());
        for Comparator {
            op,
            major,
            minor,
            patch,
            pre,
            ..
        } in self.0.comparators.iter().cloned()
        {
            match (op, minor, patch) {
                // =x.y.z
                (Op::Exact, Some(minor), Some(patch)) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(minor),
                        patch: Some(patch),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::Greater,
                        major,
                        minor: Some(minor),
                        patch: Some(patch),
                        pre,
                    },
                ]),
                // =x.y
                // x.y.*
                (Op::Exact | Op::Wildcard, Some(minor), None) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(minor),
                        patch: Some(0),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major,
                        minor: Some(minor + 1),
                        patch: Some(0),
                        pre,
                    },
                ]),
                // =x
                // x.*
                (Op::Exact | Op::Wildcard, None, None) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(0),
                        patch: Some(0),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major: major + 1,
                        minor: Some(0),
                        patch: Some(0),
                        pre,
                    },
                ]),

                // >x.y.z
                (Op::Greater, Some(minor), Some(patch)) => comparators.push(Comparator {
                    op: Op::LessEq,
                    major,
                    minor: Some(minor),
                    patch: Some(patch),
                    pre,
                }),
                // >x.y
                (Op::Greater, Some(minor), None) => comparators.push(Comparator {
                    op: Op::Less,
                    major,
                    minor: Some(minor + 1),
                    patch: Some(0),
                    pre,
                }),
                // >x
                (Op::Greater, None, None) => comparators.push(Comparator {
                    op: Op::Less,
                    major: major + 1,
                    minor: Some(0),
                    patch: Some(0),
                    pre,
                }),

                // >=x.y.z
                // >=x.y
                // >=x
                (Op::GreaterEq, minor, patch) => comparators.push(Comparator {
                    op: Op::Less,
                    major,
                    minor: Some(minor.unwrap_or(0)),
                    patch: Some(patch.unwrap_or(0)),
                    pre,
                }),

                // <x.y.z
                // <x.y
                // <x
                (Op::Less, minor, patch) => comparators.push(Comparator {
                    op: Op::GreaterEq,
                    major,
                    minor: Some(minor.unwrap_or(0)),
                    patch: Some(patch.unwrap_or(0)),
                    pre,
                }),

                // <=x.y.z
                (Op::LessEq, Some(minor), Some(patch)) => comparators.push(Comparator {
                    op: Op::Greater,
                    major,
                    minor: Some(minor),
                    patch: Some(patch),
                    pre,
                }),
                // <=x.y
                (Op::LessEq, Some(minor), None) => comparators.push(Comparator {
                    op: Op::GreaterEq,
                    major,
                    minor: Some(minor + 1),
                    patch: Some(0),
                    pre,
                }),
                // <=x
                (Op::LessEq, None, None) => comparators.push(Comparator {
                    op: Op::GreaterEq,
                    major: major + 1,
                    minor: Some(0),
                    patch: Some(0),
                    pre,
                }),

                // ~x.y.z
                (Op::Tilde, Some(minor), Some(patch)) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(minor),
                        patch: Some(patch),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major,
                        minor: Some(minor + 1),
                        patch: Some(patch),
                        pre,
                    },
                ]),
                // ~x.y
                (Op::Tilde, Some(minor), None) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(minor),
                        patch: Some(0),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major,
                        minor: Some(minor + 1),
                        patch: Some(0),
                        pre,
                    },
                ]),
                // ~x
                (Op::Tilde, None, None) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(0),
                        patch: Some(0),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major: major + 1,
                        minor: Some(0),
                        patch: Some(0),
                        pre,
                    },
                ]),

                // ^x.y.z
                // ^x.y
                // if x > 0
                (Op::Caret, Some(minor), patch) if major > 0 => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(minor),
                        patch: Some(patch.unwrap_or(0)),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major: major + 1,
                        minor: Some(0),
                        patch: Some(0),
                        pre,
                    },
                ]),
                // ^0.y.z
                // ^0.y
                // if y > 0
                (Op::Caret, Some(minor), patch) if minor > 0 => {
                    debug_assert_eq!(major, 0);
                    comparators.extend([
                        Comparator {
                            op: Op::Less,
                            major,
                            minor: Some(minor),
                            patch: Some(patch.unwrap_or(0)),
                            pre: pre.clone(),
                        },
                        Comparator {
                            op: Op::GreaterEq,
                            major,
                            minor: Some(minor + 1),
                            patch: Some(0),
                            pre,
                        },
                    ])
                }
                // ^0.0.z
                (Op::Caret, Some(minor), Some(patch)) => {
                    debug_assert_eq!(major, 0);
                    debug_assert_eq!(minor, 0);
                    comparators.extend([
                        Comparator {
                            op: Op::Less,
                            major,
                            minor: Some(minor),
                            patch: Some(patch),
                            pre: pre.clone(),
                        },
                        Comparator {
                            op: Op::Greater,
                            major,
                            minor: Some(minor),
                            patch: Some(patch),
                            pre,
                        },
                    ])
                }
                // ^0.0
                (Op::Caret, Some(minor), None) => {
                    debug_assert_eq!(major, 0);
                    debug_assert_eq!(minor, 0);
                    comparators.extend([
                        Comparator {
                            op: Op::Less,
                            major,
                            minor: Some(minor),
                            patch: Some(0),
                            pre: pre.clone(),
                        },
                        Comparator {
                            op: Op::GreaterEq,
                            major,
                            minor: Some(minor + 1),
                            patch: Some(0),
                            pre,
                        },
                    ])
                }
                // ^x
                (Op::Caret, None, None) => comparators.extend([
                    Comparator {
                        op: Op::Less,
                        major,
                        minor: Some(0),
                        patch: Some(0),
                        pre: pre.clone(),
                    },
                    Comparator {
                        op: Op::GreaterEq,
                        major: major + 1,
                        minor: Some(0),
                        patch: Some(0),
                        pre,
                    },
                ]),

                _ => unimplemented!(),
            }
        }
        Self(VersionReq { comparators })
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut s = self.clone();
        s.0.comparators.extend_from_slice(&other.0.comparators);
        s
    }

    fn contains(&self, v: &Self::V) -> bool {
        self.0.matches(&v.0)
    }

    fn full() -> Self {
        Self(VersionReq::STAR)
    }
}

macro_rules! impl_traits {
    ($($t:ty => $tt:ty),*) => {
        $(
            impl fmt::Debug for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Debug::fmt(&self.0, f)
                }
            }
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }
            impl From<$t> for $tt {
                fn from(v: $t) -> Self {
                    v.0
                }
            }
            impl From<$tt> for $t {
                fn from(v: $tt) -> Self {
                    Self(v)
                }
            }
            impl PartialEq<$tt> for $t {
                fn eq(&self, other: &$tt) -> bool {
                    self.0.eq(other)
                }
            }
        )*
    };
}
impl_traits!(Version => semver::Version, VersionSet => semver::VersionReq);
