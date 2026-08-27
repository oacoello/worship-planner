import React, { useState, useEffect, useRef } from 'react';
import {
  Title1,
  Title3,
  Button,
  makeStyles,
  tokens,
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogActions,
  DialogContent,
  Field,
  Input,
  Select,
  Badge
} from '@fluentui/react-components';
import { 
  ChevronLeftRegular, 
  ChevronRightRegular, 
  AddRegular, 
  ClockRegular, 
  ImageRegular,
  DeleteRegular,
  CalendarMonthRegular
} from '@fluentui/react-icons';
import { 
  format, 
  addMonths, 
  subMonths, 
  startOfMonth, 
  endOfMonth, 
  startOfWeek, 
  endOfWeek, 
  eachDayOfInterval, 
  isSameMonth, 
  isSameDay, 
  isToday 
} from 'date-fns';
import { es, enUS } from 'date-fns/locale';

import { getServices, saveService, getSermonSeries, saveSermonSeries } from '../db';
import { useI18n } from '../i18n';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '1300px',
    margin: '0 auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '20px',
    height: '100%',
    boxSizing: 'border-box'
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    flexWrap: 'wrap',
    gap: '16px',
  },
  navGroup: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
  },
  monthTitle: {
    fontSize: '20px',
    fontWeight: '700',
    minWidth: '180px',
    textAlign: 'center',
    color: tokens.colorNeutralForeground1,
    textTransform: 'capitalize'
  },
  calendarFrame: {
    backgroundColor: tokens.colorNeutralBackground1,
    borderRadius: tokens.borderRadiusMedium,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    overflow: 'hidden',
    boxShadow: tokens.shadow4,
    display: 'flex',
    flexDirection: 'column',
  },
  weekdaysGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(7, 1fr)',
    backgroundColor: tokens.colorNeutralBackground3,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
  },
  weekdayCell: {
    padding: '12px 8px',
    textAlign: 'center',
    fontWeight: '700',
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '0.05em',
    color: tokens.colorNeutralForeground3,
  },
  daysGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(7, 1fr)',
    gridAutoRows: 'minmax(130px, auto)',
  },
  dayCell: {
    borderRight: `1px solid ${tokens.colorNeutralStroke2}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`,
    padding: '8px',
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    minHeight: '130px',
    position: 'relative',
    backgroundColor: tokens.colorNeutralBackground1,
    transition: 'background-color 0.15s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    },
    '&:nth-child(7n)': {
      borderRight: 'none',
    }
  },
  dayCellOtherMonth: {
    backgroundColor: tokens.colorNeutralBackground2,
    opacity: 0.45,
  },
  dayHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '2px',
  },
  dayNumber: {
    fontSize: '13px',
    fontWeight: '600',
    color: tokens.colorNeutralForeground2,
    width: '24px',
    height: '24px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: '50%',
  },
  dayNumberToday: {
    backgroundColor: tokens.colorBrandBackground,
    color: tokens.colorNeutralForegroundOnBrand,
    fontWeight: '700',
  },
  addDayButton: {
    opacity: 0,
    transition: 'opacity 0.15s ease',
    '&:hover': {
      opacity: 1,
    }
  },
  serviceCardPreview: {
    backgroundColor: tokens.colorNeutralBackground2,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusSmall,
    overflow: 'hidden',
    cursor: 'pointer',
    display: 'flex',
    flexDirection: 'column',
    transition: 'all 0.15s ease',
    '&:hover': {
      transform: 'scale(1.02)',
      boxShadow: tokens.shadow8,
      borderTopColor: tokens.colorBrandStroke1,
      borderBottomColor: tokens.colorBrandStroke1,
      borderLeftColor: tokens.colorBrandStroke1,
      borderRightColor: tokens.colorBrandStroke1,
    }
  },
  serviceCardImage: {
    width: '100%',
    height: '46px',
    objectFit: 'cover',
    display: 'block',
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`
  },
  serviceCardInfo: {
    padding: '6px 8px',
    display: 'flex',
    flexDirection: 'column',
    gap: '2px',
  },
  serviceCardTitle: {
    fontSize: '12px',
    fontWeight: '700',
    color: tokens.colorNeutralForeground1,
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
  },
  serviceCardSubtitle: {
    fontSize: '11px',
    color: tokens.colorBrandForeground1,
    fontWeight: '500',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
  },
  serviceCardMeta: {
    fontSize: '10px',
    color: tokens.colorNeutralForeground3,
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    marginTop: '2px'
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    marginTop: '16px'
  },
  dropzone: {
    borderTop: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderBottom: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderLeft: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderRight: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    padding: '20px',
    textAlign: 'center',
    cursor: 'pointer',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '8px',
    color: tokens.colorNeutralForeground3,
    backgroundColor: tokens.colorNeutralBackground2,
    transition: 'all 0.2s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    }
  },
  dropzoneActive: {
    borderTop: `2px dashed ${tokens.colorBrandStroke1}`,
    borderBottom: `2px dashed ${tokens.colorBrandStroke1}`,
    borderLeft: `2px dashed ${tokens.colorBrandStroke1}`,
    borderRight: `2px dashed ${tokens.colorBrandStroke1}`,
    backgroundColor: tokens.colorBrandBackground2,
    color: tokens.colorBrandForeground1,
  },
  previewImage: {
    width: '100%',
    height: '120px',
    objectFit: 'cover',
    borderRadius: tokens.borderRadiusMedium
  },
  deleteButton: {
    color: tokens.colorPaletteRedForeground1,
    '&:hover': {
      color: tokens.colorPaletteRedForeground2,
      backgroundColor: tokens.colorPaletteRedBackground2
    }
  }
});

export default function CalendarView({ onOpenService }) {
  const styles = useStyles();
  const { t, language } = useI18n();
  const fileInputRef = useRef(null);
  
  const [currentDate, setCurrentDate] = useState(new Date());
  const [services, setServices] = useState([]);
  const [seriesList, setSeriesList] = useState([]);

  // Modal for creating service from calendar
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const [isCreatingNewSeries, setIsCreatingNewSeries] = useState(false);
  const [newSeriesData, setNewSeriesData] = useState({ title: '', artwork: null });
  const [formData, setFormData] = useState({
    title: 'Sunday Service',
    seriesId: '',
    sermonTitle: '',
    weekNumber: 'Week 1',
    date: new Date().toISOString().split('T')[0],
    time: '10:00'
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    const [svcs, sList] = await Promise.all([getServices(), getSermonSeries()]);
    setServices(svcs);
    setSeriesList(sList);
  };

  const nextMonth = () => setCurrentDate(addMonths(currentDate, 1));
  const prevMonth = () => setCurrentDate(subMonths(currentDate, 1));
  const goToToday = () => setCurrentDate(new Date());

  const monthStart = startOfMonth(currentDate);
  const monthEnd = endOfMonth(monthStart);
  const startDate = startOfWeek(monthStart);
  const endDate = endOfWeek(monthEnd);

  const days = eachDayOfInterval({ start: startDate, end: endDate });
  const dateLocale = language === 'es' ? es : enUS;

  const weekdays = [
    t('sun'),
    t('mon'),
    t('tue'),
    t('wed'),
    t('thu'),
    t('fri'),
    t('sat'),
  ];

  const openNewServiceOnDate = (date) => {
    const dateStr = format(date, 'yyyy-MM-dd');
    setFormData({
      title: 'Sunday Service',
      seriesId: '',
      sermonTitle: '',
      weekNumber: 'Week 1',
      date: dateStr,
      time: '10:00'
    });
    setIsCreatingNewSeries(false);
    setNewSeriesData({ title: '', artwork: null });
    setIsDialogOpen(true);
  };

  const processFile = (file) => {
    if (file && file.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onloadend = () => {
        setNewSeriesData(prev => ({ ...prev, artwork: reader.result }));
      };
      reader.readAsDataURL(file);
    }
  };

  const handleImageUpload = (e) => {
    const file = e.target.files?.[0];
    if (file) processFile(file);
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(true);
  };

  const handleDragLeave = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);
    const file = e.dataTransfer.files?.[0];
    if (file) processFile(file);
  };

  const handleSave = async (e) => {
    e.preventDefault();

    let chosenSeriesId = formData.seriesId;
    let chosenSeriesTitle = '';
    let chosenSeriesArt = null;

    if (isCreatingNewSeries && newSeriesData.title.trim()) {
      const createdSeries = await saveSermonSeries({
        title: newSeriesData.title.trim(),
        artwork: newSeriesData.artwork
      });
      chosenSeriesId = createdSeries.id;
      chosenSeriesTitle = createdSeries.title;
      chosenSeriesArt = createdSeries.artwork;
    } else if (chosenSeriesId) {
      const existing = seriesList.find(s => s.id === chosenSeriesId);
      if (existing) {
        chosenSeriesTitle = existing.title;
        chosenSeriesArt = existing.artwork;
      }
    }

    const payload = {
      title: formData.title,
      seriesId: chosenSeriesId,
      series: chosenSeriesTitle,
      sermonTitle: formData.sermonTitle,
      weekNumber: formData.weekNumber,
      sermonArt: chosenSeriesArt,
      date: formData.date,
      time: formData.time
    };

    await saveService(payload);
    setIsDialogOpen(false);
    loadData();
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div>
          <Title1>{t('calendarTitle')}</Title1>
          <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
            {t('calendarDesc')}
          </div>
        </div>

        <div className={styles.navGroup}>
          <Button icon={<ChevronLeftRegular />} appearance="subtle" onClick={prevMonth} />
          <div className={styles.monthTitle}>
            {format(currentDate, 'MMMM yyyy', { locale: dateLocale })}
          </div>
          <Button icon={<ChevronRightRegular />} appearance="subtle" onClick={nextMonth} />
          <Button appearance="secondary" size="small" onClick={goToToday}>
            {t('today')}
          </Button>
          <Button appearance="primary" icon={<AddRegular />} onClick={() => openNewServiceOnDate(new Date())}>
            {t('newService')}
          </Button>
        </div>
      </div>

      <div className={styles.calendarFrame}>
        {/* Weekday Columns */}
        <div className={styles.weekdaysGrid}>
          {weekdays.map((w, idx) => (
            <div key={idx} className={styles.weekdayCell}>
              {w}
            </div>
          ))}
        </div>

        {/* Days Grid */}
        <div className={styles.daysGrid}>
          {days.map((day, idx) => {
            const isCurrMonth = isSameMonth(day, monthStart);
            const isDayToday = isToday(day);
            const dayFormattedStr = format(day, 'yyyy-MM-dd');
            
            // Services on this day
            const dayServices = services.filter(s => s.date === dayFormattedStr);

            return (
              <div 
                key={idx} 
                className={`${styles.dayCell} ${!isCurrMonth ? styles.dayCellOtherMonth : ''}`}
                onMouseEnter={(e) => {
                  const btn = e.currentTarget.querySelector('.day-add-btn');
                  if (btn) btn.style.opacity = '1';
                }}
                onMouseLeave={(e) => {
                  const btn = e.currentTarget.querySelector('.day-add-btn');
                  if (btn) btn.style.opacity = '0';
                }}
              >
                <div className={styles.dayHeader}>
                  <div className={`${styles.dayNumber} ${isDayToday ? styles.dayNumberToday : ''}`}>
                    {format(day, 'd')}
                  </div>
                  
                  <Button 
                    className={`day-add-btn ${styles.addDayButton}`}
                    size="small" 
                    appearance="subtle" 
                    icon={<AddRegular fontSize={12} />} 
                    style={{ minWidth: 0, padding: '2px 4px', height: '20px' }}
                    onClick={() => openNewServiceOnDate(day)}
                    title={t('addServiceForDay')}
                  />
                </div>

                {/* Service Cards for Day */}
                {dayServices.map(service => {
                  const linkedSeries = service.seriesId ? seriesList.find(s => s.id === service.seriesId) : null;
                  const art = (linkedSeries && linkedSeries.artwork) || service.sermonArt;
                  const seriesName = (linkedSeries && linkedSeries.title) || service.series;

                  return (
                    <div 
                      key={service.id} 
                      className={styles.serviceCardPreview}
                      onClick={() => onOpenService(service.id)}
                      title={`${service.title} - ${service.time}`}
                    >
                      {art && (
                        <img src={art} alt={seriesName || 'Series Banner'} className={styles.serviceCardImage} />
                      )}
                      <div className={styles.serviceCardInfo}>
                        <div className={styles.serviceCardTitle}>{service.title}</div>
                        {seriesName && (
                          <div className={styles.serviceCardSubtitle}>{seriesName}</div>
                        )}
                        <div className={styles.serviceCardMeta}>
                          <ClockRegular fontSize={11} />
                          <span>{service.time}</span>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            );
          })}
        </div>
      </div>

      {/* New Service Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={(e, data) => {
        setIsDialogOpen(data.open);
        if (!data.open) {
          setIsCreatingNewSeries(false);
          setNewSeriesData({ title: '', artwork: null });
        }
      }}>
        <DialogSurface>
          <form onSubmit={handleSave}>
            <DialogBody>
              <DialogTitle>{t('createNewService')}</DialogTitle>
              <DialogContent className={styles.form}>
                <Field label={t('serviceName')} required>
                  <Input 
                    value={formData.title} 
                    onChange={e => setFormData({ ...formData, title: e.target.value })} 
                    placeholder="e.g. Sunday Morning Worship" 
                    required 
                  />
                </Field>

                {!isCreatingNewSeries ? (
                  <div>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '4px' }}>
                      <label style={{ fontSize: '14px', fontWeight: '600' }}>{t('sermonSeries')}</label>
                      <Button 
                        appearance="subtle" 
                        size="small" 
                        icon={<AddRegular fontSize={12} />} 
                        onClick={() => setIsCreatingNewSeries(true)}
                      >
                        {t('createNewSeries')}
                      </Button>
                    </div>
                    <Select 
                      value={formData.seriesId} 
                      onChange={e => setFormData({ ...formData, seriesId: e.target.value })}
                      style={{ width: '100%' }}
                    >
                      <option value="">{t('standaloneNoSeries')}</option>
                      {seriesList.map(s => (
                        <option key={s.id} value={s.id}>{s.title}</option>
                      ))}
                    </Select>
                    {formData.seriesId && (
                      <div style={{ marginTop: '8px' }}>
                        {(() => {
                          const sel = seriesList.find(s => s.id === formData.seriesId);
                          return sel && sel.artwork ? (
                            <img src={sel.artwork} className={styles.previewImage} alt="Series Artwork" />
                          ) : null;
                        })()}
                      </div>
                    )}
                  </div>
                ) : (
                  <div style={{ borderTop: `1px solid ${tokens.colorNeutralStroke1}`, borderBottom: `1px solid ${tokens.colorNeutralStroke1}`, borderLeft: `1px solid ${tokens.colorNeutralStroke1}`, borderRight: `1px solid ${tokens.colorNeutralStroke1}`, padding: '16px', borderRadius: tokens.borderRadiusMedium, backgroundColor: tokens.colorNeutralBackground2 }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
                      <span style={{ fontWeight: '600', fontSize: '13px' }}>{t('sermonSeries')}</span>
                      <Button 
                        appearance="subtle" 
                        size="small" 
                        onClick={() => {
                          setIsCreatingNewSeries(false);
                          setNewSeriesData({ title: '', artwork: null });
                        }}
                      >
                        {t('cancelPickExisting')}
                      </Button>
                    </div>
                    
                    <Field label={t('seriesTitle')} required style={{ marginBottom: '12px' }}>
                      <Input 
                        value={newSeriesData.title} 
                        onChange={e => setNewSeriesData({ ...newSeriesData, title: e.target.value })} 
                        placeholder="e.g. Grace Upon Grace" 
                        required={isCreatingNewSeries}
                      />
                    </Field>

                    <Field label={t('seriesArtworkGraphic')}>
                      <input 
                        type="file" 
                        accept="image/*" 
                        style={{ display: 'none' }} 
                        ref={fileInputRef}
                        onChange={handleImageUpload}
                      />
                      {!newSeriesData.artwork ? (
                        <div 
                          className={`${styles.dropzone} ${isDraggingOver ? styles.dropzoneActive : ''}`} 
                          onClick={() => fileInputRef.current?.click()}
                          onDragOver={handleDragOver}
                          onDragLeave={handleDragLeave}
                          onDrop={handleDrop}
                        >
                          <ImageRegular fontSize={28} />
                          <div style={{ fontSize: '12px' }}>{t('dropzoneText')}</div>
                        </div>
                      ) : (
                        <div style={{ position: 'relative' }}>
                          <img src={newSeriesData.artwork} className={styles.previewImage} alt="Series Preview" />
                          <Button 
                            appearance="subtle" 
                            className={styles.deleteButton}
                            style={{ position: 'absolute', top: 4, right: 4, backgroundColor: 'rgba(0,0,0,0.6)' }} 
                            icon={<DeleteRegular />} 
                            onClick={() => setNewSeriesData({ ...newSeriesData, artwork: null })}
                          />
                        </div>
                      )}
                    </Field>
                  </div>
                )}

                <div style={{ display: 'flex', gap: '16px' }}>
                  <Field label={t('sermonTitleTopic')} style={{ flex: 2 }}>
                    <Input 
                      value={formData.sermonTitle} 
                      onChange={e => setFormData({ ...formData, sermonTitle: e.target.value })} 
                      placeholder="e.g. Walking in Freedom" 
                    />
                  </Field>
                  <Field label={t('weekPart')} style={{ flex: 1 }}>
                    <Input 
                      value={formData.weekNumber} 
                      onChange={e => setFormData({ ...formData, weekNumber: e.target.value })} 
                      placeholder="e.g. Week 3" 
                    />
                  </Field>
                </div>

                <div style={{ display: 'flex', gap: '16px' }}>
                  <Field label={t('date')} required style={{ flex: 1 }}>
                    <Input type="date" value={formData.date} onChange={e => setFormData({ ...formData, date: e.target.value })} required />
                  </Field>
                  <Field label={t('time')} required style={{ flex: 1 }}>
                    <Input type="time" value={formData.time} onChange={e => setFormData({ ...formData, time: e.target.value })} required />
                  </Field>
                </div>
              </DialogContent>
              <DialogActions>
                <DialogTrigger disableButtonEnhancement>
                  <Button appearance="secondary">{t('cancel')}</Button>
                </DialogTrigger>
                <Button type="submit" appearance="primary">{t('createService')}</Button>
              </DialogActions>
            </DialogBody>
          </form>
        </DialogSurface>
      </Dialog>
    </div>
  );
}
