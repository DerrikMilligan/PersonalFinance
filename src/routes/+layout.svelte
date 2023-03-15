<script>
  // Your selected Skeleton theme:
  // import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
  // import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
  import '@skeletonlabs/skeleton/themes/theme-modern.css';
  // import '@skeletonlabs/skeleton/themes/theme-crimson.css';
  // This contains the bulk of Skeletons required styles:
  import "@skeletonlabs/skeleton/styles/all.css";
	// Most of your app wide CSS should be put in this file
  import '../app.postcss';

  import {
    autoModeWatcher,
    AppShell,
    AppBar,
    LightSwitch,
  } from '@skeletonlabs/skeleton'
</script>

<svelte:head>
  <!-- https://www.skeleton.dev/utilities/lightswitches -->
  {@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
</svelte:head>

<AppShell>
  <svelte:fragment slot="header">
		<AppBar background="bg-surface-100-800-token">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Skeleton</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
        <LightSwitch />
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://github.com/skeletonlabs/skeleton"
					target="_blank"
					rel="noreferrer"
				>
					GitHub
				</a>
			</svelte:fragment>
		</AppBar>
  </svelte:fragment>

  <slot />
</AppShell>

