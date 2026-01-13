pub const fn catloaf() -> &'static ::core::primitive::str {
    "core-affine, thread-local, opinionated actor framework"
}

/*
FOOTNOTES
- Prioritize static dispatch, which is currently used
- Consider SoA over AoS if possible
- API need to make some sense on its own, for both small and large projects
- Actor-per-thread-per-core
- Static/fixed execution graph/topology
*/
