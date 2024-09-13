<script>
    import { selectedFeed } from "$stores/stores.js";
    import { invoke } from "@tauri-apps/api/core";

    let feedData;
    const selectedFeedSubscribe = selectedFeed.subscribe(async (feedId) => {
        console.log('feedId',feedId);
        if (feedId) {
            let articles = await invoke("get_articles_by_feed", {
                feedId: feedId,
            });
            console.log("articles", articles);
            feedData = articles[0].content;
        }

        // if (feedTitle) {
        //     let data = await invoke("display_feed", { url: feedTitle });
        //     if (data) {
        //         console.log(data);
        //         feedData = data.entries[0].content;
        //         // console.log(feedData.entries);
        //     }
        // }
    });

    import { onDestroy } from "svelte";
    onDestroy(selectedFeedSubscribe);
</script>

<div>{@html feedData}</div>
