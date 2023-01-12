import React from "react";
import "./App.css";
import Cooperation from "./assets/cooperation.svg";
import Grid from "./Grid";

class App extends React.PureComponent {
	constructor(props) {
		super(props);

		/* Refs */
		this.grid = React.createRef();
	}

	/*  */
	componentDidMount () {
	}

	render () {
		return (
			<main>
				<div className="half"></div>
				<img src={Cooperation} alt="title" className="title" />

				<div className="grid">
					<Grid />
				</div>
			</main>
		);
	};
}

export default App;
