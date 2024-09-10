<script>
    let dialog;
</script>
<dialog bind:this={dialog}>
    <h2>ADD RSS</h2>
    <button class="closeModal" on:click={dialog.close()}>X</button>
    <form>
        <input type="text" />
        <button type="submit">Submit</button>
    </form>
</dialog>
<button class="openModal" on:click={dialog.showModal()}>Open</button>
