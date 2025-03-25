(function() {
    var type_impls = Object.fromEntries([["zink",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Asm-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#89-99\">Source</a><a href=\"#impl-Asm-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"zink/trait.Asm.html\" title=\"trait zink::Asm\">Asm</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.push\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#91-93\">Source</a><a href=\"#method.push\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/trait.Asm.html#tymethod.push\" class=\"fn\">push</a>(self)</h4></section></summary><div class='docblock'>Push self on the stack.</div></details><section id=\"method.bytes32\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#96-98\">Source</a><a href=\"#method.bytes32\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/trait.Asm.html#tymethod.bytes32\" class=\"fn\">bytes32</a>(&amp;self) -&gt; [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">32</a>]</h4></section></div></details>","Asm","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-Clone-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-Debug-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3Cu64%3E-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#115-129\">Source</a><a href=\"#impl-From%3Cu64%3E-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#116-128\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u64.html\">u64</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<u64>","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-PartialEq-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialOrd-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-PartialOrd-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.partial_cmp\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#method.partial_cmp\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp\" class=\"fn\">partial_cmp</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html\" title=\"enum core::cmp::Ordering\">Ordering</a>&gt;</h4></section></summary><div class='docblock'>This method returns an ordering between <code>self</code> and <code>other</code> values if one exists. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1382\">Source</a></span><a href=\"#method.lt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt\" class=\"fn\">lt</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests less than (for <code>self</code> and <code>other</code>) and is used by the <code>&lt;</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.le\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1400\">Source</a></span><a href=\"#method.le\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le\" class=\"fn\">le</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests less than or equal to (for <code>self</code> and <code>other</code>) and is used by the\n<code>&lt;=</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1418\">Source</a></span><a href=\"#method.gt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt\" class=\"fn\">gt</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests greater than (for <code>self</code> and <code>other</code>) and is used by the <code>&gt;</code>\noperator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ge\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1436\">Source</a></span><a href=\"#method.ge\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge\" class=\"fn\">ge</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests greater than or equal to (for <code>self</code> and <code>other</code>) and is used by\nthe <code>&gt;=</code> operator. <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge\">Read more</a></div></details></div></details>","PartialOrd","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SafeNumeric-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#171-217\">Source</a><a href=\"#impl-SafeNumeric-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"zink/primitives/numeric/trait.SafeNumeric.html\" title=\"trait zink::primitives::numeric::SafeNumeric\">SafeNumeric</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.max\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#173-175\">Source</a><a href=\"#method.max\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.max\" class=\"fn\">max</a>() -&gt; Self</h4></section><section id=\"method.min\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#177-179\">Source</a><a href=\"#method.min\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.min\" class=\"fn\">min</a>() -&gt; Self</h4></section><section id=\"method.safe_add\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#182-188\">Source</a><a href=\"#method.safe_add\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.safe_add\" class=\"fn\">safe_add</a>(self, rhs: Self) -&gt; Self</h4></section><section id=\"method.safe_sub\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#191-197\">Source</a><a href=\"#method.safe_sub\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.safe_sub\" class=\"fn\">safe_sub</a>(self, rhs: Self) -&gt; Self</h4></section><section id=\"method.safe_mul\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#200-208\">Source</a><a href=\"#method.safe_mul\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.safe_mul\" class=\"fn\">safe_mul</a>(self, rhs: Self) -&gt; Self</h4></section><section id=\"method.safe_div\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/numeric.rs.html#211-216\">Source</a><a href=\"#method.safe_div\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/primitives/numeric/trait.SafeNumeric.html#tymethod.safe_div\" class=\"fn\">safe_div</a>(self, rhs: Self) -&gt; Self</h4></section></div></details>","SafeNumeric","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StorageValue-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#101-106\">Source</a><a href=\"#impl-StorageValue-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"zink/storage/trait.StorageValue.html\" title=\"trait zink::storage::StorageValue\">StorageValue</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.sload\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#103-105\">Source</a><a href=\"#method.sload\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/storage/trait.StorageValue.html#tymethod.sload\" class=\"fn\">sload</a>() -&gt; Self</h4></section></summary><div class='docblock'>Load from storage</div></details></div></details>","StorageValue","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Sub-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#79-87\">Source</a><a href=\"#impl-Sub-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.sub\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#84-86\">Source</a><a href=\"#method.sub\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub\" class=\"fn\">sub</a>(self, other: Self) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output\" title=\"type core::ops::arith::Sub::Output\">Output</a></h4></section></summary><div class=\"docblock\"><p>u256 sub</p>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#80\">Source</a><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h4></section></summary><div class='docblock'>The resulting type after applying the <code>-</code> operator.</div></details></div></details>","Sub","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TransientStorageValue-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#108-113\">Source</a><a href=\"#impl-TransientStorageValue-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"zink/storage/trait.TransientStorageValue.html\" title=\"trait zink::storage::TransientStorageValue\">TransientStorageValue</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.tload\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#110-112\">Source</a><a href=\"#method.tload\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"zink/storage/trait.TransientStorageValue.html#tymethod.tload\" class=\"fn\">tload</a>() -&gt; Self</h4></section></summary><div class='docblock'>Load from transient storage</div></details></div></details>","TransientStorageValue","zink::primitives::String32"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#16-77\">Source</a><a href=\"#impl-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.empty\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#18-20\">Source</a><h4 class=\"code-header\">pub const fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.empty\" class=\"fn\">empty</a>() -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Returns empty value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#24-26\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.add\" class=\"fn\">add</a>(self, other: Self) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>u256 add</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lt\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#30-32\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.lt\" class=\"fn\">lt</a>(self, other: Self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>u256 less than</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#36-38\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.eq\" class=\"fn\">eq</a>(self, other: Self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>u256 eq</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.sub\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#42-44\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.sub\" class=\"fn\">sub</a>(self, other: Self) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>u256 sub</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.div\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#48-50\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.div\" class=\"fn\">div</a>(self, other: Self) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>u256 div</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.max\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#54-56\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.max\" class=\"fn\">max</a>() -&gt; Self</h4></section></summary><div class=\"docblock\"><p>max of u256</p>\n</div></details><section id=\"method.to_bytes32\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#58-60\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.to_bytes32\" class=\"fn\">to_bytes32</a>(&amp;self) -&gt; <a class=\"struct\" href=\"zink/primitives/bytes/struct.Bytes32.html\" title=\"struct zink::primitives::bytes::Bytes32\">Bytes32</a></h4></section><section id=\"method.bytes32\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#63-65\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.bytes32\" class=\"fn\">bytes32</a>(&amp;self) -&gt; [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">32</a>]</h4></section><section id=\"method.addmod\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#68-70\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.addmod\" class=\"fn\">addmod</a>(self, other: Self, modulus: Self) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.mulmod\" class=\"method\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#74-76\">Source</a><h4 class=\"code-header\">pub fn <a href=\"zink/primitives/u256/struct.U256.html#tymethod.mulmod\" class=\"fn\">mulmod</a>(self, other: Self, modulus: Self) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Mulmod for U256</p>\n</div></details></div></details>",0,"zink::primitives::String32"],["<section id=\"impl-Copy-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-Copy-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section>","Copy","zink::primitives::String32"],["<section id=\"impl-StructuralPartialEq-for-U256\" class=\"impl\"><a class=\"src rightside\" href=\"src/zink/primitives/u256.rs.html#13\">Source</a><a href=\"#impl-StructuralPartialEq-for-U256\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"zink/primitives/u256/struct.U256.html\" title=\"struct zink::primitives::u256::U256\">U256</a></h3></section>","StructuralPartialEq","zink::primitives::String32"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[28419]}