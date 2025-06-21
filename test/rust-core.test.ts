import assert from "node:assert/strict";
import test from "node:test";

// eslint-disable-next-line @typescript-eslint/no-var-requires
const rust = require("../rust-core");

void test("simple trade via rust core", () => {
	rust.clearBook();
	const order1 = { id: 1, price: 100, qty: 1 };
	const order2 = { id: 2, price: 100, qty: 1 };
	rust.placeLimit("ask", order1);
	const trade = rust.placeLimit("bid", order2);
	assert.ok(trade);
	assert.equal(trade.price, 100);
	assert.equal(trade.qty, 1);
});
