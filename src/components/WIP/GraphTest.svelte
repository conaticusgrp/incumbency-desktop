<!-- NOT INTENDED TO USE IN THE PROJECT -->
<!-- IS TO BE DELETED AS SOON AS THE GRAPHS ARE BEING USED -->

<script>
	import Graph from "./Graph.svelte"
	
	let data = [
		{ key: 1, value: 20 },
		{ key: 2, value: 40 },
		{ key: 3, value: 45 },
		{ key: 4, value: 60 },
		{ key: 5, value: 100 }
	];
</script>

<main>
	<Graph
				 size={{ width: 300, height: 200 }}
				 values={data}
				 yAxisName="Test name"
				 yRange={[0, 100]}
				 yValuesCount={5}
	/>
</main>

<style>

	main {
		width: 100%;
		height: 100%;
	}
	
</style>