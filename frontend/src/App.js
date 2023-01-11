import React from "react";
import "./App.css";

class App extends React.PureComponent {
	render () {
		return (
			<main>
				<div className="row" style={{ alignItems: "center" }}>
					<h1>The</h1>
					<div className="dash"></div>
					<h1>Art</h1>
					<div className="dash"></div>
					<h1>Of</h1>
				</div>
				<div className="row">
					<h1>Co</h1>
					<div style={{ width: "100%" }}>
						<div className="stripes">
							<div className="stripe"></div>
							<div className="stripe"></div>
						</div>
					</div>
					<h1>operation</h1>
				</div>
			</main>
		);
	};
}

export default App;
