// import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Mopro',
  tagline: 'Making client-side proving on mobile simple.',
  favicon: 'img/logo_sm.svg',

  // Set the production url of your site here
  url: 'https://zkmopro.org',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'zkmopro', // Usually your GitHub org/user name.
  projectName: 'mopro', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
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
            'https://github.com/zkmopro/mopro/tree/main/mopro-web',
        },
        blog: {
          showReadingTime: true,
          editUrl:
            'https://github.com/zkmopro/mopro/tree/main/mopro-web',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    // Replace with your project's social card
    image: 'img/logo_title.svg',
    navbar: {
      style: 'dark',
      // title: 'Mopro',
      logo: {
        alt: 'Mopro Logo',
        src: 'img/logo_title.svg',
        width: '325',
        height: '80',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'right',
          label: 'Docs',
        },
        { to: '/blog', label: 'Blog', position: 'right' },
        // {
        //   href: 'https://github.com/zkmopro/mopro',
        //   label: 'GitHub',
        //   position: 'right',
        // },
        {
          type: 'html',
          position: 'right',
          value: 
            `<div class="custom-navbar-link">
              <a href="https://github.com/zkmopro/mopro" target="_blank" class="navbar__link">Github</a
              ><img src="img/link_arrow.svg"/>
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
                    <img src="img/link_arrow.svg"/>
                  </div>`
            },
            // {
            //   label: 'GitHub',
            //   href: 'https://github.com/zkmopro/mopro',
            // },
            {
              label: 'Blog',
              to: '/blog',
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
                    <img src="img/link_arrow.svg"/>
                  </div>`
            },
            {
              html: 
                `<div class="custom-footer-link">
                    <a href="https://twitter.com/zkmopro" target="_blank" class="footer__link-item">Twitter</a>
                    <img src="img/link_arrow.svg"/>
                  </div>`
            },
            // {
            //   label: 'Telegram',
            //   href: 'https://t.me/zkmopro',
            // },
            // {
            //   label: 'Twitter',
            //   href: 'https://twitter.com/zkmopro',
            // },
          ],
        },
      ],
      logo: {
        alt: 'Mopro Logo',
        src: 'img/logo_lg.svg',
      },
      copyright: `Copyright © ${new Date().getFullYear()} Mopro`,
    },
    // prism: {
    //   theme: prismThemes.github,
    //   darkTheme: prismThemes.dracula,
    // },
  } satisfies Preset.ThemeConfig,
};

export default config;
