import React, { useState } from 'react';
import { 
  TabList, 
  Tab, 
  makeStyles, 
  tokens, 
  typographyStyles 
} from '@fluentui/react-components';
import { 
  CalendarLtrRegular, 
  CalendarMonthRegular,
  MusicNote1Regular, 
  ImageRegular, 
  DataTrendingRegular,
  SettingsRegular 
} from '@fluentui/react-icons';

import ServicesList from './components/ServicesList';
import CalendarView from './components/CalendarView';
import SongBank from './components/SongBank';
import SongAnalytics from './components/SongAnalytics';
import ServiceBuilder from './components/ServiceBuilder';
import SermonSeries from './components/SermonSeries';
import Settings from './components/Settings';
import { useI18n } from './i18n';

const useStyles = makeStyles({
  root: {
    display: 'flex',
    height: '100vh',
    width: '100vw',
    backgroundColor: tokens.colorNeutralBackground2,
  },
  sidebar: {
    width: '190px',
    minWidth: '190px',
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    backgroundColor: tokens.colorNeutralBackground1,
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
    padding: '16px 0',
    boxSizing: 'border-box'
  },
  sidebarTop: {
    display: 'flex',
    flexDirection: 'column',
  },
  sidebarHeader: {
    padding: '0 16px 20px',
    ...typographyStyles.subtitle1,
    fontWeight: '700',
    letterSpacing: '-0.02em',
    color: tokens.colorNeutralForeground1,
    display: 'flex',
    alignItems: 'center',
    gap: '8px'
  },
  sidebarBottom: {
    borderTop: `1px solid ${tokens.colorNeutralStroke2}`,
    paddingTop: '8px',
  },
  content: {
    flex: 1,
    overflow: 'auto',
    backgroundColor: tokens.colorNeutralBackground2,
  },
  tabList: {
    flexDirection: 'column',
    alignItems: 'stretch',
    gap: '4px'
  },
  tab: {
    justifyContent: 'flex-start',
    padding: '10px 16px',
    fontSize: '13px',
  }
});

function App() {
  const styles = useStyles();
  const { t } = useI18n();
  const [currentView, setCurrentView] = useState('services');
  const [activeServiceId, setActiveServiceId] = useState(null);

  const handleTabSelect = (event, data) => {
    setCurrentView(data.value);
    if (data.value !== 'builder') {
      setActiveServiceId(null);
    }
  };

  const openServiceBuilder = (id) => {
    setActiveServiceId(id);
    setCurrentView('builder');
  };

  return (
    <div className={styles.root}>
      <div className={`${styles.sidebar} no-print`}>
        {/* Top Section: Header & Main Navigation Items */}
        <div className={styles.sidebarTop}>
          <div className={styles.sidebarHeader}>
            {t('appTitle')}
          </div>
          <TabList 
            selectedValue={currentView === 'builder' ? 'services' : currentView} 
            onTabSelect={handleTabSelect} 
            vertical 
            className={styles.tabList}
          >
            <Tab value="services" icon={<CalendarLtrRegular />} className={styles.tab}>{t('services')}</Tab>
            <Tab value="calendar" icon={<CalendarMonthRegular />} className={styles.tab}>{t('calendar')}</Tab>
            <Tab value="series" icon={<ImageRegular />} className={styles.tab}>{t('series')}</Tab>
            <Tab value="songbank" icon={<MusicNote1Regular />} className={styles.tab}>{t('songBank')}</Tab>
            <Tab value="stats" icon={<DataTrendingRegular />} className={styles.tab}>{t('stats')}</Tab>
          </TabList>
        </div>

        {/* Bottom Section: Settings Pinned to Bottom */}
        <div className={styles.sidebarBottom}>
          <TabList 
            selectedValue={currentView} 
            onTabSelect={handleTabSelect} 
            vertical 
            className={styles.tabList}
          >
            <Tab value="settings" icon={<SettingsRegular />} className={styles.tab}>{t('settings')}</Tab>
          </TabList>
        </div>
      </div>

      <div className={styles.content}>
        {currentView === 'services' && <ServicesList onOpenService={openServiceBuilder} />}
        {currentView === 'calendar' && <CalendarView onOpenService={openServiceBuilder} />}
        {currentView === 'series' && <SermonSeries />}
        {currentView === 'builder' && <ServiceBuilder serviceId={activeServiceId} onBack={() => setCurrentView('services')} />}
        {currentView === 'songbank' && <SongBank />}
        {currentView === 'stats' && <SongAnalytics />}
        {currentView === 'settings' && <Settings />}
      </div>
    </div>
  );
}

export default App;
