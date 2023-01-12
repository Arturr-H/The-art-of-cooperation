import React, { Component } from 'react';

export default class Grid extends Component {
	constructor(props) {
		super(props);
		this.canvasRef = React.createRef();
		this.state = {
			zoom: 1
		};
	}

	handleScroll = (event) => {
		const delta = event.deltaY;
		this.setState(prevState => {
			return { zoom: prevState.zoom + delta * 0.001 }
		});
	}

	drawGrid = (zoom) => {
		const ctx = this.canvasRef.current.getContext('2d');
		ctx.clearRect(0, 0, this.canvasRef.current.width, this.canvasRef.current.height);
		ctx.setTransform(zoom, 0, 0, zoom, 0, 0);
		for (let x = 0; x < this.canvasRef.current.width; x += 20) {
			ctx.moveTo(x, 0);
			ctx.lineTo(x, this.canvasRef.current.height);
		}
		for (let y = 0; y < this.canvasRef.current.height; y += 20) {
			ctx.moveTo(0, y);
			ctx.lineTo(this.canvasRef.current.width, y);
		}
		ctx.stroke();
	}

	componentDidMount() {
		this.drawGrid(this.state.zoom);
	}


	componentDidUpdate(prevProps, prevState) {
		if (prevState.zoom !== this.state.zoom) {
			this.drawGrid(this.state.zoom);
		}
	}

	render() {
		return (
			<canvas
				ref={this.canvasRef}
				width={800}
				height={600}
				onWheel={this.handleScroll}
			/>
		);
	}
}

