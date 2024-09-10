<script>
    let count = 0;
    $: doubled = count * 2;
    $: quadrupled = doubled * 2;

    function increment() {
        count++;
    }
</script>

<button on:click={increment}>
    {count}
</button>
<p>doubled = {doubled}</p>
<p>quadrupled = {quadrupled}</p>

<style>
</style>
