import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: 'doc',
      label: 'Introduction',
      id: 'intro',
    },
    {
      type: 'doc',
      label: 'Prerequisites',
      id: 'prerequisites',
    },
    {
      type: 'doc',
      label: 'Getting Started',
      id: 'getting-started',
    },
    {
      type: 'category',
      label: 'Guides',
      items: [
        {
          type: 'doc',
          label: 'mopro-configuration',
          id: 'guides/mopro-configuration',
        },
        {
          type: 'doc',
          label: 'mopro-cli',
          id: 'guides/mopro-cli',
        },
        {
          type: 'doc',
          label: 'mopro-core',
          id: 'guides/mopro-core',
        },
        {
          type: 'doc',
          label: 'mopro-ffi',
          id: 'guides/mopro-ffi',
        },
        {
          type: 'doc',
          label: 'mopro-ios',
          id: 'guides/mopro-ios',
        },
        {
          type: 'doc',
          label: 'mopro-android',
          id: 'guides/mopro-android',
        },
      ],
    },
    {
      type: 'doc',
      label: 'Performance and Benchmarks',
      id: 'performance',
    },
    {
      type: 'doc',
      label: 'Community and Talks',
      id: 'community',
    },
    {
      type: 'doc',
      label: 'FAQ',
      id: 'FAQ',
    },
  ],
};

export default sidebars;