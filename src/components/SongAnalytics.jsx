import React, { useState, useEffect, useMemo } from 'react';
import {
  Title1,
  Title2,
  Title3,
  Card,
  makeStyles,
  tokens,
  Input,
  Select,
  Badge,
  ProgressBar
} from '@fluentui/react-components';
import { 
  MusicNote1Regular, 
  SearchRegular, 
  CalendarLtrRegular,
  TrophyRegular,
  TimerRegular,
  ChartMultipleRegular,
  HourglassHalfRegular
} from '@fluentui/react-icons';
import { format, parseISO, isAfter, subDays, subMonths, startOfYear } from 'date-fns';
import { es, enUS } from 'date-fns/locale';

import { getSongs, getServices, getAllItems } from '../db';
import { useI18n } from '../i18n';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '1200px',
    margin: '0 auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '24px',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    flexWrap: 'wrap',
    gap: '16px',
  },
  kpiGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))',
    gap: '16px',
  },
  kpiCard: {
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    padding: '20px',
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
  },
  kpiHeader: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    color: tokens.colorNeutralForeground3,
    fontSize: '13px',
    fontWeight: '600',
  },
  kpiValue: {
    fontSize: '28px',
    fontWeight: '800',
    color: tokens.colorNeutralForeground1,
    letterSpacing: '-0.02em',
  },
  controlsRow: {
    display: 'flex',
    gap: '16px',
    alignItems: 'center',
    flexWrap: 'wrap',
  },
  searchInput: {
    flex: 1,
    minWidth: '260px',
    maxWidth: '400px',
  },
  section: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
  },
  tableCard: {
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    overflow: 'hidden',
  },
  tableHeader: {
    display: 'grid',
    gridTemplateColumns: '48px 2fr 1.5fr 100px 1.5fr 140px',
    padding: '12px 20px',
    backgroundColor: tokens.colorNeutralBackground3,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    fontWeight: '700',
    fontSize: '11px',
    textTransform: 'uppercase',
    letterSpacing: '0.06em',
    color: tokens.colorNeutralForeground3,
    alignItems: 'center',
  },
  tableRow: {
    display: 'grid',
    gridTemplateColumns: '48px 2fr 1.5fr 100px 1.5fr 140px',
    padding: '14px 20px',
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`,
    alignItems: 'center',
    transition: 'background-color 0.15s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
    '&:last-child': {
      borderBottom: 'none',
    }
  },
  rankBadgeGold: {
    backgroundColor: 'rgba(234, 179, 8, 0.2)',
    color: '#eab308',
    borderTop: '1px solid #eab308',
    borderBottom: '1px solid #eab308',
    borderLeft: '1px solid #eab308',
    borderRight: '1px solid #eab308',
    width: '28px',
    height: '28px',
    borderRadius: '50%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontWeight: '800',
    fontSize: '13px',
  },
  rankBadgeSilver: {
    backgroundColor: 'rgba(148, 163, 184, 0.2)',
    color: '#94a3b8',
    borderTop: '1px solid #94a3b8',
    borderBottom: '1px solid #94a3b8',
    borderLeft: '1px solid #94a3b8',
    borderRight: '1px solid #94a3b8',
    width: '28px',
    height: '28px',
    borderRadius: '50%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontWeight: '800',
    fontSize: '13px',
  },
  rankBadgeBronze: {
    backgroundColor: 'rgba(217, 119, 6, 0.2)',
    color: '#d97706',
    borderTop: '1px solid #d97706',
    borderBottom: '1px solid #d97706',
    borderLeft: '1px solid #d97706',
    borderRight: '1px solid #d97706',
    width: '28px',
    height: '28px',
    borderRadius: '50%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontWeight: '800',
    fontSize: '13px',
  },
  rankBadgeRegular: {
    color: tokens.colorNeutralForeground3,
    width: '28px',
    height: '28px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontWeight: '600',
    fontSize: '13px',
  }
});

export default function SongAnalytics() {
  const styles = useStyles();
  const { t, language } = useI18n();
  
  const [songs, setSongs] = useState([]);
  const [services, setServices] = useState([]);
  const [items, setItems] = useState([]);
  const [timeFilter, setTimeFilter] = useState('all'); // 'all', '90days', 'year'
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    const [sngs, svcs, itms] = await Promise.all([
      getSongs(),
      getServices(),
      getAllItems()
    ]);
    setSongs(sngs);
    setServices(svcs);
    setItems(itms);
  };

  const dateLocale = language === 'es' ? es : enUS;

  // Filter services by time range
  const filteredServicesMap = useMemo(() => {
    const now = new Date();
    const map = new Map();

    services.forEach(svc => {
      let include = true;
      if (svc.date) {
        const svcDate = new Date(svc.date);
        if (timeFilter === '90days') {
          include = isAfter(svcDate, subDays(now, 90));
        } else if (timeFilter === 'year') {
          include = isAfter(svcDate, startOfYear(now));
        }
      }
      if (include) {
        map.set(svc.id, svc);
      }
    });

    return map;
  }, [services, timeFilter]);

  // Aggregate song usage metrics
  const analyticsData = useMemo(() => {
    // Map song title (case-insensitive) or match with song bank
    const usageMap = new Map();
    const keyDistribution = new Map();
    let totalScheduledSongCount = 0;

    // Initialize all songs in bank with 0 count
    songs.forEach(s => {
      const normalizedTitle = s.title.trim().toLowerCase();
      usageMap.set(normalizedTitle, {
        id: s.id,
        title: s.title,
        artist: s.artist || '',
        count: 0,
        keys: new Set(),
        lastDate: null,
        arrangementsUsed: new Set(),
        servicesScheduled: []
      });
    });

    // Process all song items from active services
    items.forEach(item => {
      if (item.type !== 'song') return;
      if (!filteredServicesMap.has(item.serviceId)) return;

      const svc = filteredServicesMap.get(item.serviceId);
      const normalizedTitle = (item.title || '').trim().toLowerCase();
      totalScheduledSongCount++;

      // Track key distribution
      if (item.key) {
        keyDistribution.set(item.key, (keyDistribution.get(item.key) || 0) + 1);
      }

      if (!usageMap.has(normalizedTitle)) {
        usageMap.set(normalizedTitle, {
          id: item.id,
          title: item.title,
          artist: item.subtitle || '',
          count: 0,
          keys: new Set(),
          lastDate: null,
          arrangementsUsed: new Set(),
          servicesScheduled: []
        });
      }

      const record = usageMap.get(normalizedTitle);
      record.count += 1;
      if (item.key) record.keys.add(item.key);
      if (item.arrangementName) record.arrangementsUsed.add(item.arrangementName);
      
      if (svc && svc.date) {
        record.servicesScheduled.push(svc);
        if (!record.lastDate || new Date(svc.date) > new Date(record.lastDate)) {
          record.lastDate = svc.date;
        }
      }
    });

    const songList = Array.from(usageMap.values());
    const maxCount = songList.reduce((max, s) => Math.max(max, s.count), 1);

    // Sort descending by count
    const sorted = [...songList].sort((a, b) => {
      if (b.count !== a.count) return b.count - a.count;
      return a.title.localeCompare(b.title);
    });

    // Top Key
    let topKey = '—';
    let topKeyCount = 0;
    keyDistribution.forEach((cnt, k) => {
      if (cnt > topKeyCount) {
        topKeyCount = cnt;
        topKey = k;
      }
    });

    return {
      songList: sorted,
      maxCount,
      totalServices: filteredServicesMap.size,
      totalScheduledSongCount,
      uniqueSongsUsed: sorted.filter(s => s.count > 0).length,
      topSong: sorted[0]?.count > 0 ? sorted[0] : null,
      topKey
    };
  }, [songs, items, filteredServicesMap]);

  // Filter songs based on search query
  const filteredList = useMemo(() => {
    return analyticsData.songList.filter(s => 
      s.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (s.artist && s.artist.toLowerCase().includes(searchQuery.toLowerCase()))
    );
  }, [analyticsData.songList, searchQuery]);

  const mostPlayedList = filteredList.filter(s => s.count > 0);
  const leastPlayedList = filteredList.filter(s => s.count === 0);

  return (
    <div className={styles.container}>
      {/* Header */}
      <div className={styles.header}>
        <div>
          <Title1>{t('analyticsTitle')}</Title1>
          <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
            {t('analyticsDesc')}
          </div>
        </div>

        {/* Time Filter */}
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <Select 
            value={timeFilter} 
            onChange={(e) => setTimeFilter(e.target.value)}
          >
            <option value="all">{language === 'es' ? 'Todo el Historial' : 'All Time'}</option>
            <option value="90days">{language === 'es' ? 'Últimos 90 días' : 'Last 90 Days'}</option>
            <option value="year">{language === 'es' ? 'Este Año' : 'This Year'}</option>
          </Select>
        </div>
      </div>

      {/* KPI Cards */}
      <div className={styles.kpiGrid}>
        <Card className={styles.kpiCard}>
          <div className={styles.kpiHeader}>
            <CalendarLtrRegular fontSize={18} />
            <span>{t('totalServicesScheduled')}</span>
          </div>
          <div className={styles.kpiValue}>{analyticsData.totalServices}</div>
        </Card>

        <Card className={styles.kpiCard}>
          <div className={styles.kpiHeader}>
            <MusicNote1Regular fontSize={18} />
            <span>{t('totalSongInstances')}</span>
          </div>
          <div className={styles.kpiValue}>{analyticsData.totalScheduledSongCount}</div>
        </Card>

        <Card className={styles.kpiCard}>
          <div className={styles.kpiHeader}>
            <ChartMultipleRegular fontSize={18} />
            <span>{t('uniqueSongsUsed')}</span>
          </div>
          <div className={styles.kpiValue}>{analyticsData.uniqueSongsUsed}</div>
        </Card>

        <Card className={styles.kpiCard}>
          <div className={styles.kpiHeader}>
            <TrophyRegular fontSize={18} />
            <span>{t('mostFrequentSong')}</span>
          </div>
          <div style={{ fontSize: '18px', fontWeight: '700', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', color: tokens.colorBrandForeground1 }}>
            {analyticsData.topSong ? `${analyticsData.topSong.title} (${analyticsData.topSong.count})` : '—'}
          </div>
        </Card>

        <Card className={styles.kpiCard}>
          <div className={styles.kpiHeader}>
            <TimerRegular fontSize={18} />
            <span>{t('mostFrequentKey')}</span>
          </div>
          <div className={styles.kpiValue}>
            <Badge appearance="tint" color="brand" size="large">{analyticsData.topKey}</Badge>
          </div>
        </Card>
      </div>

      {/* Search Input */}
      <div className={styles.controlsRow}>
        <Input 
          className={styles.searchInput}
          contentBefore={<SearchRegular />}
          placeholder={t('searchSongs')}
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
        />
      </div>

      {/* Most Frequently Scheduled Songs Table */}
      <div className={styles.section}>
        <Title3>{t('mostPlayedTitle')} ({mostPlayedList.length})</Title3>
        
        <div className={styles.tableCard}>
          <div className={styles.tableHeader}>
            <div>#</div>
            <div>{t('songTitleCol')}</div>
            <div>{t('artistCol')}</div>
            <div>{t('usageCountCol')}</div>
            <div>{t('frequencyCol')}</div>
            <div>{t('lastDateCol')}</div>
          </div>

          {mostPlayedList.map((song, index) => {
            const ratio = song.count / (analyticsData.maxCount || 1);
            let rankBadge = <div className={styles.rankBadgeRegular}>{index + 1}</div>;
            if (index === 0) rankBadge = <div className={styles.rankBadgeGold}>1</div>;
            if (index === 1) rankBadge = <div className={styles.rankBadgeSilver}>2</div>;
            if (index === 2) rankBadge = <div className={styles.rankBadgeBronze}>3</div>;

            return (
              <div key={song.id || index} className={styles.tableRow}>
                <div>{rankBadge}</div>
                <div>
                  <div style={{ fontWeight: '700', fontSize: '14px', color: tokens.colorNeutralForeground1 }}>
                    {song.title}
                  </div>
                  {song.keys.size > 0 && (
                    <div style={{ display: 'flex', gap: '4px', marginTop: '4px', flexWrap: 'wrap' }}>
                      {Array.from(song.keys).map(k => (
                        <Badge key={k} appearance="outline" size="small">{k}</Badge>
                      ))}
                    </div>
                  )}
                </div>

                <div style={{ color: tokens.colorNeutralForeground3, fontSize: '13px' }}>
                  {song.artist || '—'}
                </div>

                <div>
                  <Badge appearance="filled" color={index < 3 ? 'brand' : 'informative'}>
                    {song.count} {song.count === 1 ? 'vez' : 'veces'}
                  </Badge>
                </div>

                <div style={{ paddingRight: '20px' }}>
                  <ProgressBar value={ratio} color={index < 3 ? 'brand' : 'neutral'} />
                </div>

                <div style={{ fontSize: '12px', color: tokens.colorNeutralForeground3 }}>
                  {song.lastDate ? (() => {
                    try {
                      return format(parseISO(song.lastDate), 'MMM d, yyyy', { locale: dateLocale });
                    } catch {
                      return song.lastDate;
                    }
                  })() : '—'}
                </div>
              </div>
            );
          })}

          {mostPlayedList.length === 0 && (
            <div style={{ textAlign: 'center', padding: '40px', color: tokens.colorNeutralForeground3 }}>
              {language === 'es' ? 'No se encontraron canciones registradas con uso.' : 'No songs with recorded usage found.'}
            </div>
          )}
        </div>
      </div>

      {/* Least Played / Inactive Songs */}
      {leastPlayedList.length > 0 && (
        <div className={styles.section} style={{ marginTop: '16px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <HourglassHalfRegular fontSize={20} style={{ color: tokens.colorPaletteGoldForeground1 }} />
            <Title3>{t('leastPlayedTitle')} ({leastPlayedList.length})</Title3>
          </div>
          
          <div className={styles.tableCard}>
            <div className={styles.tableHeader} style={{ gridTemplateColumns: '2fr 1.5fr 120px' }}>
              <div>{t('songTitleCol')}</div>
              <div>{t('artistCol')}</div>
              <div>{t('lastDateCol')}</div>
            </div>

            {leastPlayedList.map((song, index) => (
              <div key={song.id || index} className={styles.tableRow} style={{ gridTemplateColumns: '2fr 1.5fr 120px' }}>
                <div style={{ fontWeight: '600', fontSize: '14px', color: tokens.colorNeutralForeground2 }}>
                  {song.title}
                </div>
                <div style={{ color: tokens.colorNeutralForeground3, fontSize: '13px' }}>
                  {song.artist || '—'}
                </div>
                <div>
                  <Badge appearance="tint" color="danger">{t('neverPlayedLabel')}</Badge>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
