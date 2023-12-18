<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { navigate } from "svelte-routing";
  import { title } from "./stores";

  let key = "";
  let greetMsg = "";
  //greet function now loads course data, and sets $title to Hello {name}
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { key });
    $title = greetMsg;
  }

  //onSubmit waits for greet() to load data, then navigates to Home page
  function onSubmit() {
    greet().then(() => {
      navigate("/lib/Home", { replace: true });
    });
  }
</script>

<main class="greet-section">
  <h1>Gooey Canvas</h1>
  <div>
    <form class="row" on:submit|preventDefault={onSubmit}>
      <input
        id="greet-input"
        placeholder="Enter an API Key..."
        bind:value={key}
      />
      <button type="submit">Submit</button>
    </form>
    <p>{greetMsg}</p>
  </div>
</main>
