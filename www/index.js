import * as my_approvals from "my_approvals";

const nodeURL = document.getElementById("node");
const owner = document.getElementById("owner");
const fromBlock = document.getElementById("from_block");
const toBlock = document.getElementById("to_block");
const getButton = document.getElementById("get_approvals");
const approvalsTable = document.getElementById("approvals");

getButton.addEventListener('click', event => {
	approvalsTable.innerHTML = "";
	getButton.value = "Loading...";
	const app = my_approvals.HTTPApp.new(nodeURL.value);
	try {
		app.get_approvals_and_tokens(owner.value, fromBlock.value, toBlock.value).then(([approvals, tokens]) => {
		for (const [approval, meta] of approvals) {
			const token = tokens.get(meta.address);
			const row = approvalsTable.insertRow();

			var token_node = document.createElement('a');
			var token_text = document.createTextNode(token.symbol);
			token_node.appendChild(token_text);
			token_node.title = meta.address;
			row.insertCell().appendChild(token_node);
			row.insertCell().innerHTML = approval.spender;
			row.insertCell().innerHTML = parseInt(approval.value, 16)/(10**token.decimals);

			var tx_node = document.createElement('a');
			var tx_text = document.createTextNode(meta.transaction_hash);
			tx_node.appendChild(tx_text);
			tx_node.href = `https://etherscan.io/tx/${meta.transaction_hash}#eventlog`;
			row.insertCell().appendChild(tx_node);
		}
		getButton.value = "Refresh Approvals";
	})
	} catch (error) {
		alert(error);
	};
})
