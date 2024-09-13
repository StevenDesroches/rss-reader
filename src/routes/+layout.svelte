<script>
    import "$styles/global.scss";
    import { json } from "@sveltejs/kit";

    // import FileTree from "$lib/components/FileTree/FileTree.svelte";
    // import Button from "$lib/components/Button/Button.svelte";
    // import Dialog from "$lib/components/Dialog/Dialog.svelte";

    import { invoke } from "@tauri-apps/api/core";

    export let data;

    let feedUrl = "";
    let feeds = [];

    // async function addFeedUrl() {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //     await invoke("add_feed", { url: feedUrl });
    //     feeds = await invoke("get_feed");
    //     console.log('feeds',feeds);
    // }

    import { selectedFeed } from "$stores/stores.js";
    function selectFeed(feedId) {
        selectedFeed.set(feedId);
    }

    $: feedTitle = "";
    $: feedCategory = null;
    let feedUrlToFetch = "";
    async function fetchFeed() {
        let feed = await invoke("fetch_feed", { url: feedUrlToFetch });
        if (feed && feed.title) feedTitle = feed.title;
        console.log("feed", feed);
    }

    async function addFeed() {
        await invoke("add_feed", { url: feedUrlToFetch, title: feedTitle,  categoryId: feedCategory });
    }

    $: categoryTitle = "";
    $: categoryParentId = null;
    async function addCategory() {
        await invoke("add_category", {
            title: categoryTitle,
            parentId: categoryParentId,
        });
    }

    let feedDialog;
    let categoryDialog;
</script>

<div class="grid">
    <div class="header">header</div>
    <div class="sidebar">
        <p>Categories</p>
        {#if data}
            {#if data.categories}
                <ul>
                    {#each data.categories as category}
                        <li>
                            <button>
                                {category.title}
                            </button>
                        </li>
                    {/each}
                </ul>
            {/if}
            <p>Feeds</p>
            {#if data.feeds}
                <ul>
                    {#each data.feeds as feed}
                        <li>
                            <!-- <button on:click={selectFeed(feedTitle)}> -->
                            <button on:click={selectFeed(feed.id)}>
                                {feed.title}
                            </button>
                        </li>
                    {/each}
                </ul>
            {/if}
        {/if}

        <!-- <form class="row" on:submit|preventDefault={addFeedUrl}>
            <input
                id="greet-input"
                placeholder="Enter a name..."
                bind:value={feedUrl}
            />
            <button type="submit">Greet</button>
        </form> -->

        <dialog bind:this={feedDialog}>
            <button on:click={feedDialog.close()}>x</button>
            <form method="dialog" on:submit={addFeed}>
                <p>url</p>
                <input
                    type="text"
                    name="url"
                    bind:value={feedUrlToFetch}
                    on:input={fetchFeed}
                />
                <p>title</p>
                <input type="text" name="title" bind:value={feedTitle} />
                <p>Category</p>
                <select name="category" bind:value={feedCategory}>
                    {#each data.categories as category}
                        <option value={category.id}>
                            {category.title}
                        </option>
                    {/each}
                </select>
                <button>OK</button>
            </form>
        </dialog>
        <button on:click={feedDialog.showModal()}>FEED</button>

        <dialog bind:this={categoryDialog}>
            <button on:click={categoryDialog.close()}>x</button>
            <form method="dialog" on:submit={addCategory}>
                <p>Parent Category</p>
                <select name="parentId" bind:value={categoryParentId}>
                    {#each data.categories as category}
                        <option value={category.id}>
                            {category.title}
                        </option>
                    {/each}
                </select>
                <p>title</p>
                <input type="text" name="title" bind:value={categoryTitle} />
                <button>OK</button>
            </form>
        </dialog>
        <button on:click={categoryDialog.showModal()}>CATEGORY</button>
    </div>
    <div class="main">
        <slot />
    </div>
</div>

<style>
    .grid {
        height: 100%;
        display: grid;
        grid-template-columns: minmax(150px, auto) 8fr 2fr;
        grid-template-rows: minmax(50px, auto) 9fr 2fr;
        grid-template-areas:
            "hd hd hd"
            "side main main"
            "side main main";
    }

    .header {
        background-color: blue;
        grid-area: hd;
    }

    .sidebar {
        background-color: teal;
        grid-area: side;
    }
    .main {
        background-color: red;
        grid-area: main;
    }
</style>
