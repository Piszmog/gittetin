<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  invoke('get_notifications', { codeHost: 'GitHub', opts: { all: true } })
    .then((res) => console.log(res))
    .catch((e) => console.log(e));
</script>

<main>
  <p>foo</p>
</main>

<style>
</style>
