<script lang="ts">
  export let size: number = 24;
  export let strokeWidth: number = 2;

  let className: string = '';
  export { className as class };
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class="icon icon-tabler icon-tabler-chevron-down {className}"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  stroke-width={strokeWidth}
  stroke="currentColor"
  fill="none"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path stroke="none" d="M0 0h24v24H0z" fill="none" />
  <polyline points="6 9 12 15 18 9" />
</svg>
