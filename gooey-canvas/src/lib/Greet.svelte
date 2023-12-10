<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let key = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { key })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter an API Key..." bind:value={name} />
    <button type="submit">Submit</button>
  </form>
  <p>{greetMsg}</p>
</div>
