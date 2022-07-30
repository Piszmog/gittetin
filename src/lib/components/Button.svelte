<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  let className: string = '';
  export { className as class };

  const dispatch = createEventDispatcher();

  const click = () => {
    dispatch('click');
  };
</script>

<button on:click={click} class={className}>
  <slot />
</button>
