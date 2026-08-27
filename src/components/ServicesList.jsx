import React, { useState, useEffect, useRef } from 'react';
import {
  Title1,
  Title3,
  Button,
  Card,
  CardFooter,
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
  Select
} from '@fluentui/react-components';
import { 
  AddRegular, 
  CalendarLtrRegular, 
  ClockRegular,
  DeleteRegular, 
  EditRegular, 
  ImageRegular, 
  DocumentBulletListMultipleRegular
} from '@fluentui/react-icons';
import { getServices, saveService, deleteService, getSermonSeries, saveSermonSeries } from '../db';
import { format } from 'date-fns';
import { es, enUS } from 'date-fns/locale';
import { useI18n } from '../i18n';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '1200px',
    margin: '0 auto',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '32px',
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(320px, 1fr))',
    gap: '24px',
  },
  card: {
    padding: '0px',
    cursor: 'pointer',
    transition: 'transform 0.15s ease, box-shadow 0.15s ease',
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    overflow: 'hidden',
    display: 'flex',
    flexDirection: 'column',
    '&:hover': {
      transform: 'translateY(-3px)',
      boxShadow: tokens.shadow16,
    }
  },
  cardBanner: {
    width: '100%',
    height: '160px',
    backgroundColor: tokens.colorNeutralBackground3,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    overflow: 'hidden',
    position: 'relative',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  cardBannerImage: {
    width: '100%',
    height: '100%',
    objectFit: 'cover',
    display: 'block'
  },
  cardBannerPlaceholder: {
    width: '100%',
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: tokens.colorNeutralBackground3,
    color: tokens.colorNeutralForeground3,
    gap: '8px'
  },
  cardContent: {
    padding: '16px 16px 12px',
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    flex: 1
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

export default function ServicesList({ onOpenService }) {
  const styles = useStyles();
  const { t, language } = useI18n();
  const dateLocale = language === 'es' ? es : enUS;
  const fileInputRef = useRef(null);
  const [services, setServices] = useState([]);
  const [seriesList, setSeriesList] = useState([]);
  
  // Dialog States
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingServiceId, setEditingServiceId] = useState(null);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  
  // Form State
  const [formData, setFormData] = useState({
    title: 'Sunday Service',
    seriesId: '',
    sermonTitle: '',
    weekNumber: 'Week 1',
    date: new Date().toISOString().split('T')[0],
    time: '10:00'
  });

  // Inline new series creation state
  const [isCreatingNewSeries, setIsCreatingNewSeries] = useState(false);
  const [newSeriesData, setNewSeriesData] = useState({ title: '', artwork: null });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    const [svcs, sList] = await Promise.all([getServices(), getSermonSeries()]);
    setServices(svcs);
    setSeriesList(sList);
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

  const openNewServiceModal = () => {
    setEditingServiceId(null);
    setIsCreatingNewSeries(false);
    setNewSeriesData({ title: '', artwork: null });
    setFormData({
      title: 'Sunday Service',
      seriesId: '',
      sermonTitle: '',
      weekNumber: 'Week 1',
      date: new Date().toISOString().split('T')[0],
      time: '10:00'
    });
    setIsDialogOpen(true);
  };

  const openEditServiceModal = (e, service) => {
    e.stopPropagation();
    setEditingServiceId(service.id);
    setIsCreatingNewSeries(false);
    setNewSeriesData({ title: '', artwork: null });
    setFormData({
      title: service.title || 'Sunday Service',
      seriesId: service.seriesId || '',
      sermonTitle: service.sermonTitle || '',
      weekNumber: service.weekNumber || '',
      date: service.date || new Date().toISOString().split('T')[0],
      time: service.time || '10:00'
    });
    setIsDialogOpen(true);
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

    if (editingServiceId) {
      payload.id = editingServiceId;
    }

    await saveService(payload);

    setIsDialogOpen(false);
    setIsCreatingNewSeries(false);
    setEditingServiceId(null);
    setNewSeriesData({ title: '', artwork: null });
    loadData();
  };

  const handleDelete = async (e, id) => {
    e.stopPropagation();
    if (window.confirm(t('confirmDeleteService'))) {
      await deleteService(id);
      loadData();
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div>
          <Title1>{t('servicePlans')}</Title1>
          <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
            {t('servicePlansDesc')}
          </div>
        </div>
        
        <Button appearance="primary" icon={<AddRegular />} onClick={openNewServiceModal}>
          {t('newService')}
        </Button>
      </div>

      {/* Service Modal (Create & Edit) */}
      <Dialog open={isDialogOpen} onOpenChange={(e, data) => {
        setIsDialogOpen(data.open);
        if (!data.open) {
          setIsCreatingNewSeries(false);
          setEditingServiceId(null);
          setNewSeriesData({ title: '', artwork: null });
        }
      }}>
        <DialogSurface>
          <form onSubmit={handleSave}>
            <DialogBody>
              <DialogTitle>{editingServiceId ? t('editServicePlan') : t('createNewService')}</DialogTitle>
              <DialogContent className={styles.form}>
                <Field label={t('serviceName')} required>
                  <Input 
                    value={formData.title} 
                    onChange={e => setFormData({ ...formData, title: e.target.value })} 
                    placeholder="e.g. Sunday Morning Worship" 
                    required 
                  />
                </Field>

                {/* Sermon Series Picker */}
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
                <Button type="submit" appearance="primary">{editingServiceId ? t('saveChanges') : t('createService')}</Button>
              </DialogActions>
            </DialogBody>
          </form>
        </DialogSurface>
      </Dialog>

      {services.length === 0 ? (
        <div style={{ textAlign: 'center', padding: '64px', color: tokens.colorNeutralForeground3 }}>
          {t('noServicesFound')}
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '48px' }}>
          {(() => {
            const standalone = [];
            const grouped = {};

            services.forEach(service => {
              if (service.seriesId) {
                if (!grouped[service.seriesId]) grouped[service.seriesId] = [];
                grouped[service.seriesId].push(service);
              } else {
                standalone.push(service);
              }
            });

            const renderServiceCard = (service, linkedSeries) => {
              const art = (linkedSeries && linkedSeries.artwork) || service.sermonArt;
              const seriesName = (linkedSeries && linkedSeries.title) || service.series;

              const sermonSubtitle = [
                seriesName ? `${t('seriesPrefix')}: ${seriesName}` : '',
                service.weekNumber ? service.weekNumber : '',
                service.sermonTitle ? `"${service.sermonTitle}"` : ''
              ].filter(Boolean).join(' • ');

              return (
                <Card key={service.id} className={styles.card} onClick={() => onOpenService(service.id)}>
                  <div className={styles.cardBanner}>
                    {art ? (
                      <img src={art} alt={seriesName || 'Series Art'} className={styles.cardBannerImage} />
                    ) : (
                      <div className={styles.cardBannerPlaceholder}>
                        <DocumentBulletListMultipleRegular fontSize={36} />
                        <span style={{ fontSize: '13px', fontWeight: '500' }}>{t('standaloneWorshipService')}</span>
                      </div>
                    )}
                  </div>

                  <div className={styles.cardContent}>
                    <Title3 style={{ margin: 0, color: tokens.colorNeutralForeground1 }}>
                      {service.title || 'Service Plan'}
                    </Title3>
                    
                    {sermonSubtitle && (
                      <div style={{ color: tokens.colorBrandForeground1, fontWeight: '600', fontSize: '13px', lineHeight: '1.4' }}>
                        {sermonSubtitle}
                      </div>
                    )}

                    <div style={{ color: tokens.colorNeutralForeground3, display: 'flex', alignItems: 'center', gap: '8px', fontSize: '13px', marginTop: '6px' }}>
                      <CalendarLtrRegular fontSize={15} />
                      <span>
                        {(() => {
                          try {
                            const [year, month, day] = service.date.split('-').map(num => parseInt(num, 10));
                            const [hours, minutes] = (service.time || '10:00').split(':').map(num => parseInt(num, 10));
                            const d = new Date(year, month - 1, day, hours || 0, minutes || 0, 0);
                            const formatted = format(d, language === 'es' ? "d 'de' MMMM, yyyy • h:mm a" : 'MMMM d, yyyy • h:mm a', { locale: dateLocale });
                            return formatted.charAt(0).toUpperCase() + formatted.slice(1);
                          } catch {
                            return `${service.date} • ${service.time}`;
                          }
                        })()}
                      </span>
                    </div>
                  </div>

                  <CardFooter style={{ justifyContent: 'space-between', borderTop: `1px solid ${tokens.colorNeutralStroke2}`, padding: '10px 16px', margin: 0 }}>
                    <Button 
                      appearance="subtle" 
                      icon={<EditRegular />} 
                      onClick={(e) => openEditServiceModal(e, service)}
                    >
                      {t('editPlan')}
                    </Button>
                    <Button 
                      appearance="subtle" 
                      className={styles.deleteButton} 
                      icon={<DeleteRegular />} 
                      onClick={(e) => handleDelete(e, service.id)}
                    >
                      {t('delete')}
                    </Button>
                  </CardFooter>
                </Card>
              );
            };

            const renderedGroups = seriesList
              .filter(s => grouped[s.id] && grouped[s.id].length > 0)
              .map(series => (
                <div key={series.id}>
                  <Title3 style={{ marginBottom: '16px', display: 'block', color: tokens.colorNeutralForeground1 }}>
                    {t('seriesPrefix')}: {series.title}
                  </Title3>
                  <div className={styles.grid}>
                    {grouped[series.id].map(service => renderServiceCard(service, series))}
                  </div>
                </div>
              ));

            if (standalone.length > 0) {
              renderedGroups.push(
                <div key="standalone">
                  <Title3 style={{ marginBottom: '16px', display: 'block', color: tokens.colorNeutralForeground1 }}>
                    {t('standaloneServicesGroup')}
                  </Title3>
                  <div className={styles.grid}>
                    {standalone.map(service => renderServiceCard(service, null))}
                  </div>
                </div>
              );
            }

            return renderedGroups;
          })()}
        </div>
      )}
    </div>
  );
}
