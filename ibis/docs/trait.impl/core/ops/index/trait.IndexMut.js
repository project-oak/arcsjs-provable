(function() {var implementors = {
"indexmap":[["impl&lt;K, V, Q, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Q</a>&gt; for <a class=\"struct\" href=\"indexmap/map/struct.IndexMap.html\" title=\"struct indexmap::map::IndexMap\">IndexMap</a>&lt;K, V, S&gt;<span class=\"where fmt-newline\">where\n    Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"indexmap/trait.Equivalent.html\" title=\"trait indexmap::Equivalent\">Equivalent</a>&lt;K&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;K, V, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"indexmap/map/struct.IndexMap.html\" title=\"struct indexmap::map::IndexMap\">IndexMap</a>&lt;K, V, S&gt;"]],
"petgraph":[["impl&lt;N, E, Ty, Ix&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"struct\" href=\"petgraph/graph/struct.EdgeIndex.html\" title=\"struct petgraph::graph::EdgeIndex\">EdgeIndex</a>&lt;Ix&gt;&gt; for <a class=\"struct\" href=\"petgraph/graph/struct.Graph.html\" title=\"struct petgraph::graph::Graph\">Graph</a>&lt;N, E, Ty, Ix&gt;<span class=\"where fmt-newline\">where\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,\n    Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>,</span>"],["impl&lt;'a, G, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;I&gt; for <a class=\"struct\" href=\"petgraph/graph/struct.Frozen.html\" title=\"struct petgraph::graph::Frozen\">Frozen</a>&lt;'a, G&gt;<span class=\"where fmt-newline\">where\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;I&gt;,</span>"],["impl&lt;N, E, Ty, Ix&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;Ix&gt; for <a class=\"struct\" href=\"petgraph/csr/struct.Csr.html\" title=\"struct petgraph::csr::Csr\">Csr</a>&lt;N, E, Ty, Ix&gt;<span class=\"where fmt-newline\">where\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,\n    Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>,</span>"],["impl&lt;N, E, Ty, Ix&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"struct\" href=\"petgraph/graph/struct.EdgeIndex.html\" title=\"struct petgraph::graph::EdgeIndex\">EdgeIndex</a>&lt;Ix&gt;&gt; for <a class=\"struct\" href=\"petgraph/stable_graph/struct.StableGraph.html\" title=\"struct petgraph::stable_graph::StableGraph\">StableGraph</a>&lt;N, E, Ty, Ix&gt;<span class=\"where fmt-newline\">where\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,\n    Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>,</span>"],["impl&lt;N, E, Ty, Ix&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"struct\" href=\"petgraph/graph/struct.NodeIndex.html\" title=\"struct petgraph::graph::NodeIndex\">NodeIndex</a>&lt;Ix&gt;&gt; for <a class=\"struct\" href=\"petgraph/stable_graph/struct.StableGraph.html\" title=\"struct petgraph::stable_graph::StableGraph\">StableGraph</a>&lt;N, E, Ty, Ix&gt;<span class=\"where fmt-newline\">where\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,\n    Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>,</span>"],["impl&lt;N, E, Ty, Ix&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"struct\" href=\"petgraph/graph/struct.NodeIndex.html\" title=\"struct petgraph::graph::NodeIndex\">NodeIndex</a>&lt;Ix&gt;&gt; for <a class=\"struct\" href=\"petgraph/graph/struct.Graph.html\" title=\"struct petgraph::graph::Graph\">Graph</a>&lt;N, E, Ty, Ix&gt;<span class=\"where fmt-newline\">where\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,\n    Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>,</span>"],["impl&lt;N, E, Ty&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.tuple.html\">(N, N)</a>&gt; for <a class=\"struct\" href=\"petgraph/graphmap/struct.GraphMap.html\" title=\"struct petgraph::graphmap::GraphMap\">GraphMap</a>&lt;N, E, Ty&gt;<span class=\"where fmt-newline\">where\n    N: <a class=\"trait\" href=\"petgraph/graphmap/trait.NodeTrait.html\" title=\"trait petgraph::graphmap::NodeTrait\">NodeTrait</a>,\n    Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>,</span>"],["impl&lt;N, E, Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>, Null: <a class=\"trait\" href=\"petgraph/matrix_graph/trait.Nullable.html\" title=\"trait petgraph::matrix_graph::Nullable\">Nullable</a>&lt;Wrapped = E&gt;, Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"struct\" href=\"petgraph/graph/struct.NodeIndex.html\" title=\"struct petgraph::graph::NodeIndex\">NodeIndex</a>&lt;Ix&gt;&gt; for <a class=\"struct\" href=\"petgraph/matrix_graph/struct.MatrixGraph.html\" title=\"struct petgraph::matrix_graph::MatrixGraph\">MatrixGraph</a>&lt;N, E, Ty, Null, Ix&gt;"],["impl&lt;N, E, Ty: <a class=\"trait\" href=\"petgraph/trait.EdgeType.html\" title=\"trait petgraph::EdgeType\">EdgeType</a>, Null: <a class=\"trait\" href=\"petgraph/matrix_graph/trait.Nullable.html\" title=\"trait petgraph::matrix_graph::Nullable\">Nullable</a>&lt;Wrapped = E&gt;, Ix: <a class=\"trait\" href=\"petgraph/graph/trait.IndexType.html\" title=\"trait petgraph::graph::IndexType\">IndexType</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;(<a class=\"struct\" href=\"petgraph/graph/struct.NodeIndex.html\" title=\"struct petgraph::graph::NodeIndex\">NodeIndex</a>&lt;Ix&gt;, <a class=\"struct\" href=\"petgraph/graph/struct.NodeIndex.html\" title=\"struct petgraph::graph::NodeIndex\">NodeIndex</a>&lt;Ix&gt;)&gt; for <a class=\"struct\" href=\"petgraph/matrix_graph/struct.MatrixGraph.html\" title=\"struct petgraph::matrix_graph::MatrixGraph\">MatrixGraph</a>&lt;N, E, Ty, Null, Ix&gt;"]],
"serde_json":[["impl&lt;'a, Q&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;'a Q</a>&gt; for <a class=\"struct\" href=\"serde_json/struct.Map.html\" title=\"struct serde_json::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,\n    Q: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,</span>"],["impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;I&gt; for <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a><span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"serde_json/value/trait.Index.html\" title=\"trait serde_json::value::Index\">Index</a>,</span>"]],
"syn":[["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"syn/punctuated/struct.Punctuated.html\" title=\"struct syn::punctuated::Punctuated\">Punctuated</a>&lt;T, P&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()