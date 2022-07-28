<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import Items from './lib/components/Items.svelte';
  import type { Notification } from './lib/models/notifications';

  const notificationsPromise = invoke<Notification[]>('get_notifications', {
    codeHost: 'GitHub',
    opts: { all: true },
  });
</script>

<main>
  {#await notificationsPromise}
    <p>Loading...</p>
  {:then notifications}
    <Items {notifications} />
  {:catch err}
    <p>{err}</p>
  {/await}
</main>

<style>
</style>
