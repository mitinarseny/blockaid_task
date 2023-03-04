import * as my_approvals from "my_approvals";

const nodeURL = document.getElementById("node");
const owner = document.getElementById("owner");
const fromBlock = document.getElementById("from_block");
const toBlock = document.getElementById("to_block");
const getButton = document.getElementById("get_approvals");
const approvalsTable = document.getElementById("approvals");

var app = my_approvals.HTTPApp.new(nodeURL.value);
var last_node_url = nodeURL.value;

getButton.addEventListener('click', event => {
	approvalsTable.innerHTML = "";
	getButton.value = "Loading...";
	if (nodeURL.value != last_node_url) {
		app = my_approvals.HTTPApp.new(nodeURL.value);
		last_node_url = nodeURL.value;
	}
	try {
		app.get_token_approvals(owner.value, fromBlock.value, toBlock.value).then((approvals) => {
		for (const a of approvals) {
			const row = approvalsTable.insertRow();

			var token_node = document.createElement('a');
			var token_text = document.createTextNode(a.token.symbol);
			token_node.appendChild(token_text);
			token_node.title = a.meta.address;
			row.insertCell().appendChild(token_node);
			row.insertCell().innerHTML = a.approval.spender;
			row.insertCell().innerHTML = parseInt(a.approval.value, 16)/(10**a.token.decimals);

			var tx_node = document.createElement('a');
			var tx_text = document.createTextNode(a.meta.transaction_hash);
			tx_node.appendChild(tx_text);
			tx_node.href = `https://etherscan.io/tx/${a.meta.transaction_hash}#eventlog`;
			row.insertCell().appendChild(tx_node);
		}
		getButton.value = "Refresh Approvals";
	})
	} catch (error) {
		alert(error);
	};
})
