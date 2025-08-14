import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

const config: Config = {
  title: 'Mopro',
  tagline: 'Making client-side proving on mobile simple.',
  favicon: '/img/logo_sm.svg',
  url: 'https://zkmopro.org',
  baseUrl: '/',
  organizationName: 'zkmopro',
  projectName: 'mopro',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl:
            'https://github.com/zkmopro/mopro/tree/main/docs',
          includeCurrentVersion: true,
          lastVersion: 'current',
          versions: {
            current: {
              label: '0.2.0',
            },
          },
        },
        blog: {
          showReadingTime: true,
          editUrl:
            'https://github.com/zkmopro/mopro/tree/main/docs',
          remarkPlugins: [remarkMath],
          rehypePlugins: [rehypeKatex],
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  stylesheets: [
    {
      href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
      type: 'text/css',
      integrity:
        'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
      crossorigin: 'anonymous',
    },
  ],

  themeConfig: {
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    prism: {
      additionalLanguages: ['powershell', 'bash', 'toml', 'diff', 'yaml', 'ruby', 'dart'],
    },
    algolia: {
      appId: '57U3VFSJ7M',
      apiKey: '430a9611dea4e9f5a937a6b8c108592d',
      indexName: 'zkmopro',
      contextualSearch: true,
    },
    image: 'img/logo_title.svg',
    navbar: {
      style: 'dark',
      logo: {
        alt: 'Mopro Logo',
        src: '/img/logo_title.svg',
        width: '125',
        height: '80',
      },
      items: [
        {
          type: 'docsVersionDropdown',
          position: 'left',
          // dropdownItemsAfter: [{to: '/versions.json', label: 'All versions'}],
          dropdownActiveClassDisabled: true,
        },
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'right',
          label: 'Docs',
        },
        { to: '/blog', label: 'Blog', position: 'right' },
        {
          type: 'html',
          position: 'right',
          value:
            `<div class="custom-navbar-link">
              <a href="https://github.com/zkmopro/mopro" target="_blank" class="navbar__link">Github</a
              ><img src="/img/link_arrow.svg"/>
            </div>`

        },
      ],
    },
    footer: {
      links: [
        {
          title: 'Resources',
          items: [
            {
              html:
                `<div class="custom-footer-link">
                    <a href="https://github.com/zkmopro/mopro" target="_blank" class="footer__link-item">Github</a>
                    <img src="/img/link_arrow.svg"/>
                  </div>`
            },
            {
              label: 'Documentation',
              to: '/docs/intro',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              html:
                `<div class="custom-footer-link">
                    <a href="https://t.me/zkmopro" target="_blank" class="footer__link-item">Telegram</a>
                    <img src="/img/link_arrow.svg"/>
                  </div>`
            },
            {
              html:
                `<div class="custom-footer-link">
                    <a href="https://twitter.com/zkmopro" target="_blank" class="footer__link-item">Twitter</a>
                    <img src="/img/link_arrow.svg"/>
                  </div>`
            },
          ],
        },
      ],
      logo: {
        alt: 'Mopro Logo',
        src: 'img/logo_lg.svg',
      },
      copyright: `Copyright © ${new Date().getFullYear()} Mopro`,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
