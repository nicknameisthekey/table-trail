<div class="box">
	<slot />
</div>

<style>
	.box {
		width: auto;
		height: 100%;
		box-sizing: border-box;
		overflow: scroll;
		border: 1px solid #aaa;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1);
		padding: 1em;
		margin: 0 0 1em 0;
	}
</style>