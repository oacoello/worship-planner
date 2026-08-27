import React, { useState, useEffect, useMemo, useRef } from 'react';
import {
  Title2,
  Subtitle1,
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
  Textarea,
  Badge,
  Spinner
} from '@fluentui/react-components';
import { 
  ArrowLeftRegular, 
  PrintRegular, 
  CameraRegular, 
  MusicNote1Regular, 
  VideoRegular, 
  MegaphoneRegular, 
  ArrowRepeatAllRegular, 
  TimerRegular, 
  ClockRegular,
  CalendarLtrRegular,
  DeleteRegular,
  ReOrderDotsVerticalRegular,
  EditRegular,
  ImageRegular,
  AddRegular,
  SearchRegular,
  DismissRegular,
  CheckmarkRegular,
  PresenterRegular
} from '@fluentui/react-icons';
import { format, addSeconds, setHours, setMinutes, setSeconds } from 'date-fns';
import { es, enUS } from 'date-fns/locale';
import { DndContext, closestCenter, KeyboardSensor, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { arrayMove, SortableContext, sortableKeyboardCoordinates, verticalListSortingStrategy, useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import html2canvas from 'html2canvas';

import { v4 as uuidv4 } from 'uuid';
import { getService, saveService, getItemsForService, saveItem, deleteItem, saveItems, getSongs, saveSong, getSermonSeries, getSermonSeriesById, saveSermonSeries } from '../db';
import { useI18n } from '../i18n';
import { useAppTheme } from '../themeContext';
import { extractMeshGradient } from '../utils/colorExtractor';
import logoLight from '../assets/logo_light.png';
import logoDark from '../assets/logo_dark.png';

const useStyles = makeStyles({
  builderWrapper: {
    width: '100%',
    minHeight: '100%',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column',
    position: 'relative',
    transition: 'all 0.5s cubic-bezier(0.16, 1, 0.3, 1)',
  },
  container: {
    padding: '32px',
    maxWidth: '1200px',
    margin: '0 auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '20px',
    height: '100%',
    width: '100%',
    boxSizing: 'border-box'
  },
  toolbar: {
    display: 'flex',
    gap: '8px',
    padding: '12px 16px',
    backgroundColor: tokens.colorNeutralBackground1,
    backdropFilter: 'blur(16px)',
    borderRadius: tokens.borderRadiusMedium,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    boxShadow: tokens.shadow4,
  },
  // High-legibility Sheet Container
  sheet: {
    borderRadius: tokens.borderRadiusMedium,
    overflow: 'hidden',
    display: 'flex',
    flexDirection: 'column',
    transition: 'background-color 0.2s ease, border-color 0.2s ease',
  },
  // Top Banner area with Sermon Series Graphic on upper left
  sheetBanner: {
    display: 'flex',
    alignItems: 'center',
    padding: '24px',
    gap: '24px',
    transition: 'background-color 0.2s ease, border-color 0.2s ease',
  },
  bannerImage: {
    width: '120px',
    height: '75px',
    objectFit: 'cover',
    borderRadius: '6px',
    boxShadow: '0 4px 12px rgba(0,0,0,0.25)'
  },
  sheetHeaderInfo: {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    gap: '4px'
  },
  serviceTitleRow: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px'
  },
  serviceTitleText: {
    fontSize: '24px',
    fontWeight: '700',
    letterSpacing: '-0.02em',
    margin: 0
  },
  sermonDetailsText: {
    fontSize: '15px',
    fontWeight: '600',
  },
  metaBadgesRow: {
    display: 'flex',
    alignItems: 'center',
    gap: '16px',
    fontSize: '13px',
    marginTop: '6px'
  },
  churchLogoContainer: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'flex-end',
    marginLeft: 'auto',
    flexShrink: 0,
  },
  churchLogoImg: {
    height: '100px',
    maxWidth: '300px',
    objectFit: 'contain',
    display: 'block'
  },
  // Setlist Header: Length first, Start Time, Type Icon, Item & Arrangement, Notes, Vocalist/Speaker, Actions
  tableHeader: {
    display: 'grid',
    gridTemplateColumns: '32px 70px 80px 32px 1.2fr 1.2fr 180px 36px',
    padding: '12px 20px',
    fontWeight: '700',
    fontSize: '11px',
    textTransform: 'uppercase',
    letterSpacing: '0.06em',
    alignItems: 'center',
    transition: 'background-color 0.2s ease, color 0.2s ease',
  },
  tableBody: {
    display: 'flex',
    flexDirection: 'column',
  },
  // Row base styling
  itemRow: {
    display: 'flex',
    flexDirection: 'column',
    transition: 'background-color 0.15s ease',
  },
  itemMain: {
    display: 'grid',
    gridTemplateColumns: '32px 70px 80px 32px 1.2fr 1.2fr 180px 36px',
    alignItems: 'center',
    padding: '14px 20px',
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    marginTop: '12px'
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
  },
  songSearchContainer: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
  },
  songListScroll: {
    maxHeight: '190px',
    overflowY: 'auto',
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    backgroundColor: tokens.colorNeutralBackground1,
    display: 'flex',
    flexDirection: 'column',
  },
  songListItem: {
    padding: '10px 14px',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    cursor: 'pointer',
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`,
    transition: 'background-color 0.15s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    }
  },
  selectedSongCard: {
    padding: '12px 16px',
    borderRadius: tokens.borderRadiusMedium,
    borderTop: `1px solid ${tokens.colorBrandStroke1}`,
    borderBottom: `1px solid ${tokens.colorBrandStroke1}`,
    borderLeft: `1px solid ${tokens.colorBrandStroke1}`,
    borderRight: `1px solid ${tokens.colorBrandStroke1}`,
    backgroundColor: tokens.colorBrandBackground2,
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  }
});

function SortableItem({ id, item, startTime, onDelete, onUpdateNotes, onUpdateVocalist }) {
  const styles = useStyles();
  const { t } = useI18n();
  const { isDark } = useAppTheme();
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  
  const [isEditingNotes, setIsEditingNotes] = useState(false);
  const [localNotes, setLocalNotes] = useState(item.notes || '');

  const [isEditingVocalist, setIsEditingVocalist] = useState(false);
  const [localVocalist, setLocalVocalist] = useState(item.leadVocalist || '');

  useEffect(() => {
    setLocalNotes(item.notes || '');
  }, [item.notes]);

  useEffect(() => {
    setLocalVocalist(item.leadVocalist || '');
  }, [item.leadVocalist]);

  // Theme-aware row styling
  const getRowThemeStyle = () => {
    if (isDark) {
      switch(item.type) {
        case 'video':
          return {
            backgroundColor: '#0a2318',
            borderLeft: '4px solid #22c55e',
            borderBottom: '1px solid #163826'
          };
        case 'loop':
          return {
            backgroundColor: '#0a1d33',
            borderLeft: '4px solid #38bdf8',
            borderBottom: '1px solid #122b4a'
          };
        case 'announcements':
          return {
            backgroundColor: '#2b1a06',
            borderLeft: '4px solid #f97316',
            borderBottom: '1px solid #422608'
          };
        case 'conference':
          return {
            backgroundColor: '#1f132b',
            borderLeft: '4px solid #a855f7',
            borderBottom: '1px solid #331d47'
          };
        case 'song':
          return {
            backgroundColor: '#1c1c1c',
            borderLeft: '4px solid #525252',
            borderBottom: '1px solid #292929'
          };
        default:
          return {
            backgroundColor: '#1a1a1a',
            borderLeft: '4px solid #404040',
            borderBottom: '1px solid #262626'
          };
      }
    } else {
      // Light Mode: Clear, bright, high-contrast pastel tints
      switch(item.type) {
        case 'video':
          return {
            backgroundColor: '#f0fdf4',
            borderLeft: '4px solid #16a34a',
            borderBottom: '1px solid #dcfce7'
          };
        case 'loop':
          return {
            backgroundColor: '#eff6ff',
            borderLeft: '4px solid #2563eb',
            borderBottom: '1px solid #dbeafe'
          };
        case 'announcements':
          return {
            backgroundColor: '#fff7ed',
            borderLeft: '4px solid #ea580c',
            borderBottom: '1px solid #ffedd5'
          };
        case 'conference':
          return {
            backgroundColor: '#faf5ff',
            borderLeft: '4px solid #9333ea',
            borderBottom: '1px solid #f3e8ff'
          };
        case 'song':
          return {
            backgroundColor: '#ffffff',
            borderLeft: '4px solid #94a3b8',
            borderBottom: '1px solid #f1f5f9'
          };
        default:
          return {
            backgroundColor: '#f8fafc',
            borderLeft: '4px solid #cbd5e1',
            borderBottom: '1px solid #e2e8f0'
          };
      }
    }
  };

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
    position: 'relative',
    zIndex: isDragging ? 10 : 1,
    ...getRowThemeStyle()
  };

  const renderTypeIcon = () => {
    switch(item.type) {
      case 'video':
        return <VideoRegular fontSize={18} style={{ color: isDark ? '#4ade80' : '#16a34a' }} title="Video" />;
      case 'loop':
        return <ArrowRepeatAllRegular fontSize={18} style={{ color: isDark ? '#38bdf8' : '#2563eb' }} title="Loop" />;
      case 'announcements':
        return <MegaphoneRegular fontSize={18} style={{ color: isDark ? '#fb923c' : '#ea580c' }} title="Announcements" />;
      case 'conference':
        return <PresenterRegular fontSize={18} style={{ color: isDark ? '#c084fc' : '#9333ea' }} title="Conference" />;
      case 'song':
        return <MusicNote1Regular fontSize={18} style={{ color: isDark ? '#9ca3af' : '#64748b' }} title="Song" />;
      default:
        return <TimerRegular fontSize={18} style={{ color: isDark ? '#a3a3a3' : '#64748b' }} title={item.type} />;
    }
  };

  const formatDuration = (seconds) => {
    const mins = Math.floor((seconds || 0) / 60);
    const secs = (seconds || 0) % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const saveNotes = () => {
    onUpdateNotes(id, localNotes);
    setIsEditingNotes(false);
  };

  const saveVocalist = () => {
    onUpdateVocalist(id, localVocalist);
    setIsEditingVocalist(false);
  };

  return (
    <div ref={setNodeRef} style={style} className={`${styles.itemRow} setlist-row`}>
      <div className={`${styles.itemMain} setlist-row-main`}>
        {/* 1. Drag Handle */}
        <div {...attributes} {...listeners} style={{ cursor: 'grab', display: 'flex', alignItems: 'center', color: isDark ? '#737373' : '#94a3b8' }} className="no-print">
          <ReOrderDotsVerticalRegular />
        </div>
        
        {/* 2. Length (First column) */}
        <div className="setlist-duration-col" style={{ fontFamily: 'monospace', fontWeight: '700', fontSize: '13px', color: isDark ? '#d4d4d4' : '#475569' }}>
          {formatDuration(item.duration)}
        </div>

        {/* 3. Start Time */}
        <div className="setlist-time-col no-print-col" style={{ fontWeight: '700', fontSize: '13px', color: isDark ? '#ffffff' : '#0f172a', fontFamily: 'Segoe UI, monospace' }}>
          {format(startTime, 'h:mm a')}
        </div>

        {/* 4. Type Icon */}
        <div className="setlist-type-icon" style={{ display: 'flex', alignItems: 'center' }}>
          {renderTypeIcon()}
        </div>
        
        {/* 5. Item & Badges */}
        <div className="setlist-title-col">
          <div style={{ display: 'flex', flexDirection: 'column', gap: '3px' }}>
            <span style={{ fontWeight: '700', fontSize: '15px', color: isDark ? '#ffffff' : '#0f172a' }}>{item.title}</span>
            {(item.key || item.arrangementName) && (
              <div style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '6px' }}>
                {item.key && (
                  <span className="setlist-key-badge" style={{ 
                    fontWeight: '700', 
                    fontSize: '11px', 
                    color: isDark ? '#fef08a' : '#854d0e', 
                    backgroundColor: isDark ? 'rgba(234, 179, 8, 0.15)' : '#fef9c3', 
                    padding: '1px 6px', 
                    borderRadius: '4px', 
                    border: `1px solid ${isDark ? 'rgba(234, 179, 8, 0.3)' : '#fde047'}`,
                    width: 'fit-content'
                  }}>
                    {t('key')}: {item.key}
                  </span>
                )}
                {item.arrangementName && (
                  <span className="setlist-arrangement-badge" style={{ 
                    fontSize: '11px', 
                    fontWeight: '500', 
                    color: isDark ? '#93c5fd' : '#1e40af', 
                    backgroundColor: isDark ? 'rgba(59, 130, 246, 0.15)' : '#dbeafe', 
                    padding: '1px 6px', 
                    borderRadius: '4px', 
                    border: `1px solid ${isDark ? 'rgba(59, 130, 246, 0.3)' : '#bfdbfe'}`
                  }}>
                    {item.arrangementName}
                  </span>
                )}
              </div>
            )}
          </div>
        </div>

        {/* 6. Notes & Cues Column */}
        <div className="setlist-notes-col" style={{ fontSize: '13px', color: isDark ? '#d4d4d4' : '#334155', fontStyle: item.notes ? 'italic' : 'normal', display: 'flex', alignItems: 'center', gap: '6px', paddingRight: '8px' }}>
          <span style={{ flex: 1, whiteSpace: 'normal', wordBreak: 'break-word' }}>
            {item.notes || '—'}
          </span>
          <Button className="no-print" appearance="transparent" size="small" icon={<EditRegular fontSize={12} />} onClick={() => { setIsEditingNotes(!isEditingNotes); setIsEditingVocalist(false); }} style={{ minWidth: 0, padding: '4px', color: isDark ? '#737373' : '#94a3b8' }} title={t('saveNotes')} />
        </div>

        {/* 7. Lead Vocalist / Speaker */}
        <div className="setlist-speaker-col" style={{ fontSize: '13px', fontWeight: '600', color: isDark ? '#e5e5e5' : '#0f172a', display: 'flex', alignItems: 'center', gap: '6px' }}>
          <span style={{ flex: 1, whiteSpace: 'normal', wordBreak: 'break-word', lineHeight: '1.3' }}>
            {item.leadVocalist || '—'}
          </span>
          <Button 
            className="no-print" 
            appearance="transparent" 
            size="small" 
            icon={<EditRegular fontSize={12} />} 
            onClick={() => { setIsEditingVocalist(!isEditingVocalist); setIsEditingNotes(false); }} 
            style={{ minWidth: 0, padding: '4px', color: isDark ? '#737373' : '#94a3b8', flexShrink: 0 }} 
            title={t('mainVocalistLead')} 
          />
        </div>
        
        {/* 8. Delete Button */}
        <div className="no-print" style={{ zIndex: 10, display: 'flex', justifyContent: 'flex-end' }}>
          <Button appearance="subtle" className={styles.deleteButton} icon={<DeleteRegular />} onClick={() => onDelete(id)} />
        </div>
      </div>

      {isEditingNotes && (
        <div className="no-print" style={{ padding: '0 40px 16px 150px' }}>
          <Field label={t('extraNotesTechnicalCues')}>
            <Textarea 
              placeholder="e.g. Lead acoustic intro, Stage lights to soft blue, Mic 2 active..." 
              value={localNotes} 
              onChange={(e, data) => setLocalNotes(data.value)}
              rows={3}
              autoFocus
            />
          </Field>
          <div style={{ marginTop: '8px', display: 'flex', gap: '8px' }}>
            <Button size="small" appearance="primary" onClick={saveNotes}>{t('saveNotes')}</Button>
            <Button size="small" onClick={() => setIsEditingNotes(false)}>{t('cancel')}</Button>
          </div>
        </div>
      )}

      {isEditingVocalist && (
        <div className="no-print" style={{ padding: '0 40px 16px 150px' }}>
          <Field label={t('mainVocalistLead')}>
            <Input 
              placeholder="e.g. Sarah M., Pastor Dave" 
              value={localVocalist} 
              onChange={(e) => setLocalVocalist(e.target.value)}
              autoFocus
              onKeyDown={(e) => {
                if (e.key === 'Enter') saveVocalist();
                if (e.key === 'Escape') setIsEditingVocalist(false);
              }}
            />
          </Field>
          <div style={{ marginTop: '8px', display: 'flex', gap: '8px' }}>
            <Button size="small" appearance="primary" onClick={saveVocalist}>{t('saveChanges')}</Button>
            <Button size="small" onClick={() => setIsEditingVocalist(false)}>{t('cancel')}</Button>
          </div>
        </div>
      )}
    </div>
  );
}

export default function ServiceBuilder({ serviceId, onBack }) {
  const styles = useStyles();
  const { t, language } = useI18n();
  const { isDark } = useAppTheme();
  const editBannerFileRef = useRef(null);
  
  const [service, setService] = useState(null);
  const [seriesArt, setSeriesArt] = useState(null);
  const [seriesList, setSeriesList] = useState([]);
  const [items, setItems] = useState([]);
  const [songs, setSongs] = useState([]);
  const [meshBg, setMeshBg] = useState('');
  
  // Add item modal
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [newItemType, setNewItemType] = useState('song');
  
  // Edit Service Plan modal
  const [isEditServiceOpen, setIsEditServiceOpen] = useState(false);
  const [isCreatingNewSeries, setIsCreatingNewSeries] = useState(false);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const [newSeriesData, setNewSeriesData] = useState({ title: '', artwork: null });
  const [editServiceForm, setEditServiceForm] = useState({
    title: '',
    seriesId: '',
    sermonTitle: '',
    weekNumber: '',
    date: '',
    time: ''
  });

  // Add item form state
  const [formData, setFormData] = useState({ 
    title: '', 
    subtitle: '', 
    arrangementName: 'Standard',
    key: 'C',
    leadVocalist: '',
    notes: '',
    durationMins: 5, 
    durationSecs: 0, 
    selectedSongId: '',
    selectedArrangementId: ''
  });

  useEffect(() => {
    if (serviceId) {
      loadData();
    }
  }, [serviceId]);

  useEffect(() => {
    const art = seriesArt || service?.sermonArt;
    extractMeshGradient(art, isDark).then(gradient => {
      setMeshBg(gradient);
    });
  }, [seriesArt, service, isDark]);

  const loadData = async () => {
    const [svc, sList, itms, sngs] = await Promise.all([
      getService(serviceId),
      getSermonSeries(),
      getItemsForService(serviceId),
      getSongs()
    ]);

    setService(svc);
    setSeriesList(sList);
    setItems(itms);
    setSongs(sngs);

    if (svc && svc.seriesId) {
      const s = await getSermonSeriesById(svc.seriesId);
      if (s && s.artwork) {
        setSeriesArt(s.artwork);
      } else {
        setSeriesArt(svc.sermonArt || null);
      }
    } else if (svc) {
      setSeriesArt(svc.sermonArt || null);
    }
  };

  const openEditPlanModal = () => {
    if (!service) return;
    setEditServiceForm({
      title: service.title || '',
      seriesId: service.seriesId || '',
      sermonTitle: service.sermonTitle || '',
      weekNumber: service.weekNumber || '',
      date: service.date || '',
      time: service.time || '10:00'
    });
    setIsCreatingNewSeries(false);
    setNewSeriesData({ title: '', artwork: null });
    setIsEditServiceOpen(true);
  };

  const handleEditPlanSubmit = async (e) => {
    e.preventDefault();

    let chosenSeriesId = editServiceForm.seriesId;
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

    const updated = {
      ...service,
      title: editServiceForm.title,
      seriesId: chosenSeriesId,
      series: chosenSeriesTitle,
      sermonTitle: editServiceForm.sermonTitle,
      weekNumber: editServiceForm.weekNumber,
      sermonArt: chosenSeriesArt,
      date: editServiceForm.date,
      time: editServiceForm.time
    };

    await saveService(updated);
    setIsEditServiceOpen(false);
    loadData();
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

  const serviceStartTime = useMemo(() => {
    if (!service || !service.date) return new Date();
    try {
      const timeStr = service.time || '10:00';
      const [year, month, day] = service.date.split('-').map(num => parseInt(num, 10));
      const [hours, minutes] = timeStr.split(':').map(num => parseInt(num, 10));
      return new Date(year, month - 1, day, hours || 0, minutes || 0, 0);
    } catch {
      return new Date();
    }
  }, [service]);

  let currentTime = serviceStartTime;
  let totalDurationSeconds = 0;

  const itemsWithTimes = items.map((item) => {
    const itemWithTime = { ...item, startTime: currentTime };
    const dur = item.duration || 0;
    currentTime = addSeconds(currentTime, dur);
    totalDurationSeconds += dur;
    return itemWithTime;
  });

  const estimatedEndTime = currentTime;

  const totalFormattedTime = useMemo(() => {
    const hours = Math.floor(totalDurationSeconds / 3600);
    const mins = Math.floor((totalDurationSeconds % 3600) / 60);
    if (hours > 0) return `${hours}h ${mins}m`;
    return `${mins} min`;
  }, [totalDurationSeconds]);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  );

  const handleDragEnd = async (event) => {
    const { active, over } = event;
    if (active.id !== over.id) {
      const oldIndex = items.findIndex(i => i.id === active.id);
      const newIndex = items.findIndex(i => i.id === over.id);
      const newItems = arrayMove(items, oldIndex, newIndex);
      
      setItems(newItems);
      
      const updatedItems = newItems.map((item, index) => ({ ...item, order: index }));
      await saveItems(updatedItems);
    }
  };

  const handleDelete = async (id) => {
    await deleteItem(id);
    loadData();
  };

  const handleUpdateNotes = async (id, notes) => {
    const item = items.find(i => i.id === id);
    if (item) {
      const updated = { ...item, notes };
      await saveItem(updated);
      setItems(items.map(i => i.id === id ? updated : i));
    }
  };

  const handleUpdateVocalist = async (id, leadVocalist) => {
    const item = items.find(i => i.id === id);
    if (item) {
      const updated = { ...item, leadVocalist };
      await saveItem(updated);
      setItems(items.map(i => i.id === id ? updated : i));
    }
  };

  const [songSearchQuery, setSongSearchQuery] = useState('');

  const filteredSongs = useMemo(() => {
    if (!songSearchQuery.trim()) return songs;
    const q = songSearchQuery.toLowerCase().trim();
    return songs.filter(s => 
      (s.title || '').toLowerCase().includes(q) || 
      (s.artist || '').toLowerCase().includes(q)
    );
  }, [songs, songSearchQuery]);

  const openAddModal = (type) => {
    setNewItemType(type);
    setSongSearchQuery('');
    let defaultDurationMins = 5;
    if (type === 'video') defaultDurationMins = 3;
    if (type === 'loop') defaultDurationMins = 4;
    if (type === 'announcements') defaultDurationMins = 4;
    if (type === 'conference') defaultDurationMins = 35;
    if (type === 'countdown') defaultDurationMins = 5;

    setFormData({ 
      title: '', 
      subtitle: '', 
      arrangementName: 'Standard',
      key: 'C',
      leadVocalist: '',
      notes: '',
      durationMins: defaultDurationMins, 
      durationSecs: 0, 
      selectedSongId: '',
      selectedArrangementId: ''
    });
    setIsModalOpen(true);
  };

  const handleSelectSongFromSearch = (song) => {
    const defaultArr = song?.arrangements?.[0];
    setFormData(prev => ({
      ...prev,
      selectedSongId: song.id,
      selectedArrangementId: defaultArr ? defaultArr.id : '',
      title: song.title,
      subtitle: song.artist || '',
      key: defaultArr?.key || song.defaultKey || 'C',
      arrangementName: defaultArr?.name || 'Standard',
      durationMins: Math.floor((defaultArr?.duration || song.defaultDuration || 300) / 60),
      durationSecs: (defaultArr?.duration || song.defaultDuration || 300) % 60,
    }));
  };

  const handleClearSelectedSong = () => {
    setFormData(prev => ({
      ...prev,
      selectedSongId: '',
      selectedArrangementId: '',
      title: '',
      subtitle: '',
      key: 'C',
      arrangementName: 'Standard',
      durationMins: 5,
      durationSecs: 0,
      leadVocalist: '',
      notes: ''
    }));
  };

  const handleArrangementSelect = (arrId) => {
    const song = songs.find(s => s.id === formData.selectedSongId);
    const arr = (song?.arrangements || []).find(a => a.id === arrId);
    if (arr) {
      setFormData(prev => ({
        ...prev,
        selectedArrangementId: arrId,
        arrangementName: arr.name,
        key: arr.key || 'C',
        durationMins: Math.floor((arr.duration || 300) / 60),
        durationSecs: (arr.duration || 300) % 60
      }));
    }
  };

  const handleAddItem = async (e) => {
    e.preventDefault();
    
    let title = formData.title;
    let subtitle = formData.subtitle;
    let arrangementName = formData.arrangementName;
    let key = formData.key;
    let leadVocalist = formData.leadVocalist;
    let duration = parseInt(formData.durationMins || 0) * 60 + parseInt(formData.durationSecs || 0);

    if (newItemType === 'song') {
      if (!formData.selectedSongId) {
        return;
      }
      const song = songs.find(s => s.id === formData.selectedSongId);
      if (song) {
        title = song.title;
        subtitle = song.artist || '';
        const arr = (song.arrangements || []).find(a => a.id === formData.selectedArrangementId) || song.arrangements?.[0];
        if (arr) {
          arrangementName = arr.name;
          key = arr.key || key;
        }
      }
    }

    const newItem = {
      serviceId,
      type: newItemType,
      title,
      subtitle,
      arrangementName: newItemType === 'song' ? (arrangementName || 'Standard') : '',
      key: newItemType === 'song' ? (key || 'C') : '',
      leadVocalist: leadVocalist || '',
      duration,
      order: items.length,
      notes: formData.notes
    };

    await saveItem(newItem);
    setIsModalOpen(false);
    loadData();
  };

  const takeScreenshot = () => {
    const element = document.getElementById('print-area');
    if (!element) return;

    const elementsToHide = element.querySelectorAll('.no-print');
    elementsToHide.forEach(el => {
      el.style.visibility = 'hidden';
    });
    
    html2canvas(element, { 
      backgroundColor: isDark ? '#181818' : '#ffffff', 
      scale: 2,
      useCORS: true,
      logging: false
    }).then(canvas => {
      const link = document.createElement('a');
      const filename = `${service.title || 'Setlist'}-${service.date}.png`.replace(/\s+/g, '-');
      link.download = filename;
      link.href = canvas.toDataURL('image/png');
      link.click();
      
      elementsToHide.forEach(el => {
        el.style.visibility = '';
      });
    }).catch(err => {
      console.error('Screenshot error:', err);
      elementsToHide.forEach(el => {
        el.style.visibility = '';
      });
    });
  };

  if (!service) return <Spinner style={{ padding: '64px' }} />;

  const selectedSong = songs.find(s => s.id === formData.selectedSongId);
  const dateLocale = language === 'es' ? es : enUS;

  return (
    <div className={`${styles.builderWrapper} setlist-builder-wrapper`} style={{ backgroundImage: meshBg }}>
      <div className={`${styles.container} setlist-main-container`}>
        <div className="no-print" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Button icon={<ArrowLeftRegular />} appearance="subtle" onClick={onBack}>
            {t('backToServices')}
          </Button>
          <div style={{ display: 'flex', gap: '8px' }}>
            <Button icon={<EditRegular />} onClick={openEditPlanModal}>
              {t('editPlanDetails')}
            </Button>
            <Button icon={<PrintRegular />} onClick={() => window.print()}>
              {t('printRunSheet')}
            </Button>
            <Button icon={<CameraRegular />} appearance="primary" onClick={takeScreenshot}>
              {t('savePngScreenshot')}
            </Button>
          </div>
        </div>

        {/* Action Toolbar */}
        <div className={`${styles.toolbar} no-print`}>
          <Button icon={<MusicNote1Regular />} onClick={() => openAddModal('song')}>{t('addSong')}</Button>
          <Button icon={<PresenterRegular />} onClick={() => openAddModal('conference')}>{t('addConference')}</Button>
          <Button icon={<VideoRegular />} onClick={() => openAddModal('video')}>{t('addVideo')}</Button>
          <Button icon={<ArrowRepeatAllRegular />} onClick={() => openAddModal('loop')}>{t('addLoop')}</Button>
          <Button icon={<MegaphoneRegular />} onClick={() => openAddModal('announcements')}>{t('addAnnouncements')}</Button>
          <Button icon={<TimerRegular />} onClick={() => openAddModal('countdown')}>{t('addCountdown')}</Button>
        </div>

        {/* The Printable & Screenshot Sheet */}
        <div 
          id="print-area" 
          className={`${styles.sheet} setlist-sheet`}
          style={{
            backgroundColor: isDark ? '#181818' : '#ffffff',
            border: `1px solid ${isDark ? '#2d2d2d' : '#e2e8f0'}`,
            color: isDark ? '#ffffff' : '#0f172a',
            boxShadow: isDark 
              ? '0 12px 30px rgba(0, 0, 0, 0.5), 0 2px 6px rgba(0, 0, 0, 0.3)' 
              : '0 10px 25px -5px rgba(0, 0, 0, 0.08), 0 8px 10px -6px rgba(0, 0, 0, 0.04)'
          }}
        >
          {/* Banner with sermon series graphic in upper left */}
          <div 
            className={`${styles.sheetBanner} setlist-banner`}
            style={{
              backgroundColor: isDark ? '#141414' : '#f8fafc',
              borderBottom: `2px solid ${isDark ? '#262626' : '#e2e8f0'}`
            }}
          >
            {seriesArt && (
              <img 
                src={seriesArt} 
                className={`${styles.bannerImage} setlist-banner-image`} 
                alt="Sermon Series Banner"
                style={{
                  border: `1px solid ${isDark ? '#333333' : '#e2e8f0'}`
                }}
              />
            )}
            <div className={`${styles.sheetHeaderInfo} setlist-header-info`}>
              <div className={`${styles.serviceTitleRow} setlist-title-row`}>
                <h1 className={`${styles.serviceTitleText} setlist-title`} style={{ color: isDark ? '#ffffff' : '#0f172a' }}>
                  {service.title || 'Sunday Worship Service'}
                </h1>
                <Button 
                  className="no-print" 
                  appearance="subtle" 
                  size="small" 
                  icon={<EditRegular fontSize={14} />} 
                  onClick={openEditPlanModal} 
                  title={t('editPlanDetails')}
                />
              </div>
              
              {(service.series || service.sermonTitle || service.weekNumber) && (
                <div className={`${styles.sermonDetailsText} setlist-details`} style={{ color: isDark ? '#38bdf8' : '#0284c7' }}>
                  {[
                    service.series ? `${t('seriesPrefix')}: ${service.series}` : '',
                    service.weekNumber ? service.weekNumber : '',
                    service.sermonTitle ? `"${service.sermonTitle}"` : ''
                  ].filter(Boolean).join(' • ')}
                </div>
              )}

              <div className={`${styles.metaBadgesRow} setlist-meta`} style={{ color: isDark ? '#a3a3a3' : '#64748b' }}>
                <span className="setlist-meta-item" style={{ display: 'inline-flex', alignItems: 'center', gap: '4px' }}>
                  <CalendarLtrRegular fontSize={14} />
                  {(() => {
                    try {
                      const formatted = format(serviceStartTime, language === 'es' ? "EEEE, d 'de' MMMM, yyyy" : 'EEEE, MMMM d, yyyy', { locale: dateLocale });
                      return formatted.charAt(0).toUpperCase() + formatted.slice(1);
                    } catch {
                      return service.date;
                    }
                  })()}
                </span>
                <span className="setlist-meta-item" style={{ display: 'inline-flex', alignItems: 'center', gap: '4px' }}>
                  <ClockRegular fontSize={14} />
                  {t('start')}: <strong style={{ color: isDark ? '#ffffff' : '#0f172a' }}>{format(serviceStartTime, 'h:mm a')}</strong>
                </span>
                <span className="setlist-meta-item" style={{ display: 'inline-flex', alignItems: 'center', gap: '4px' }}>
                  <TimerRegular fontSize={14} />
                  {t('total')}: <strong style={{ color: isDark ? '#ffffff' : '#0f172a' }}>{totalFormattedTime}</strong> ({t('estEnd')}: {format(estimatedEndTime, 'h:mm a')})
                </span>
              </div>
            </div>

            {/* Church Logo on Right */}
            <div className={`${styles.churchLogoContainer} setlist-church-logo-container`}>
              <img 
                src={isDark ? logoLight : logoDark} 
                alt="Church Logo" 
                className={`${styles.churchLogoImg} screen-only-logo`} 
              />
              <img 
                src={logoDark} 
                alt="Church Logo" 
                className="print-only-logo" 
              />
            </div>
          </div>

          {/* Setlist Table Header */}
          <div 
            className={`${styles.tableHeader} setlist-table-header`}
            style={{
              backgroundColor: isDark ? '#202020' : '#f1f5f9',
              borderBottom: `1px solid ${isDark ? '#2d2d2d' : '#e2e8f0'}`,
              color: isDark ? '#a3a3a3' : '#475569'
            }}
          >
            <div className="no-print"></div>
            <div className="setlist-th-length">{t('length')}</div>
            <div className="setlist-th-start no-print-col">{t('start')}</div>
            <div className="setlist-th-icon"></div>
            <div className="setlist-th-item">
              <span className="screen-only-text">{t('itemArrangement')}</span>
              <span className="print-only-text">{language === 'es' ? 'ELEMENTO' : 'ITEM'}</span>
            </div>
            <div className="setlist-th-notes">{t('notesCues')}</div>
            <div className="setlist-th-vocalist">{t('leadVocalist')}</div>
            <div className="no-print"></div>
          </div>
        
        {/* Setlist Rows */}
        <div className={`${styles.tableBody} setlist-table-body`}>
          <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
            <SortableContext items={items} strategy={verticalListSortingStrategy}>
              {itemsWithTimes.map((item) => (
                <SortableItem 
                  key={item.id} 
                  id={item.id} 
                  item={item} 
                  startTime={item.startTime} 
                  onDelete={handleDelete}
                  onUpdateNotes={handleUpdateNotes}
                  onUpdateVocalist={handleUpdateVocalist}
                />
              ))}
            </SortableContext>
          </DndContext>
          
          {items.length === 0 && (
            <div style={{ textAlign: 'center', padding: '64px', color: '#737373' }}>
              {t('emptySheetPrompt')}
            </div>
          )}
        </div>
      </div>

      {/* Edit Service Plan Modal */}
      <Dialog open={isEditServiceOpen} onOpenChange={(e, data) => {
        setIsEditServiceOpen(data.open);
        if (!data.open) {
          setIsCreatingNewSeries(false);
          setNewSeriesData({ title: '', artwork: null });
        }
      }}>
        <DialogSurface>
          <form onSubmit={handleEditPlanSubmit}>
            <DialogBody>
              <DialogTitle>{t('editServicePlan')}</DialogTitle>
              <DialogContent className={styles.form}>
                <Field label={t('serviceName')} required>
                  <Input 
                    value={editServiceForm.title} 
                    onChange={e => setEditServiceForm({ ...editServiceForm, title: e.target.value })} 
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
                      value={editServiceForm.seriesId} 
                      onChange={e => setEditServiceForm({ ...editServiceForm, seriesId: e.target.value })}
                      style={{ width: '100%' }}
                    >
                      <option value="">{t('standaloneNoSeries')}</option>
                      {seriesList.map(s => (
                        <option key={s.id} value={s.id}>{s.title}</option>
                      ))}
                    </Select>
                    {editServiceForm.seriesId && (
                      <div style={{ marginTop: '8px' }}>
                        {(() => {
                          const sel = seriesList.find(s => s.id === editServiceForm.seriesId);
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
                        ref={editBannerFileRef}
                        onChange={handleImageUpload}
                      />
                      {!newSeriesData.artwork ? (
                        <div 
                          className={`${styles.dropzone} ${isDraggingOver ? styles.dropzoneActive : ''}`} 
                          onClick={() => editBannerFileRef.current?.click()}
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
                      value={editServiceForm.sermonTitle} 
                      onChange={e => setEditServiceForm({ ...editServiceForm, sermonTitle: e.target.value })} 
                      placeholder="e.g. Walking in Freedom" 
                    />
                  </Field>
                  <Field label={t('weekPart')} style={{ flex: 1 }}>
                    <Input 
                      value={editServiceForm.weekNumber} 
                      onChange={e => setEditServiceForm({ ...editServiceForm, weekNumber: e.target.value })} 
                      placeholder="e.g. Week 3" 
                    />
                  </Field>
                </div>

                <div style={{ display: 'flex', gap: '16px' }}>
                  <Field label={t('date')} required style={{ flex: 1 }}>
                    <Input type="date" value={editServiceForm.date} onChange={e => setEditServiceForm({ ...editServiceForm, date: e.target.value })} required />
                  </Field>
                  <Field label={t('time')} required style={{ flex: 1 }}>
                    <Input type="time" value={editServiceForm.time} onChange={e => setEditServiceForm({ ...editServiceForm, time: e.target.value })} required />
                  </Field>
                </div>
              </DialogContent>
              <DialogActions>
                <DialogTrigger disableButtonEnhancement>
                  <Button appearance="secondary">{t('cancel')}</Button>
                </DialogTrigger>
                <Button type="submit" appearance="primary">{t('saveChanges')}</Button>
              </DialogActions>
            </DialogBody>
          </form>
        </DialogSurface>
      </Dialog>

      {/* Add Item Modal */}
      <Dialog open={isModalOpen} onOpenChange={(e, data) => setIsModalOpen(data.open)}>
        <DialogSurface>
          <form onSubmit={handleAddItem}>
            <DialogBody>
              <DialogTitle>
                {newItemType === 'song' ? t('addSong') : newItemType === 'conference' ? t('addConference') : newItemType === 'loop' ? t('addLoop') : newItemType === 'video' ? t('addVideo') : newItemType === 'announcements' ? t('addAnnouncements') : t('addCountdown')}
              </DialogTitle>
              <DialogContent className={styles.form}>
                {newItemType === 'song' ? (
                  <>
                    {!formData.selectedSongId ? (
                      /* Song Search and Selection Interface */
                      <div className={styles.songSearchContainer}>
                        <Field label={t('chooseFromSongBank')} required>
                          <Input 
                            contentBefore={<SearchRegular />}
                            value={songSearchQuery}
                            onChange={(e) => setSongSearchQuery(e.target.value)}
                            placeholder={t('searchSongs')}
                            autoFocus
                          />
                        </Field>

                        <div className={styles.songListScroll}>
                          {filteredSongs.length > 0 ? (
                            filteredSongs.map((song) => (
                              <div 
                                key={song.id} 
                                className={styles.songListItem}
                                onClick={() => handleSelectSongFromSearch(song)}
                              >
                                <div>
                                  <div style={{ fontWeight: '600', fontSize: '14px', color: isDark ? '#ffffff' : '#0f172a' }}>
                                    {song.title}
                                  </div>
                                  <div style={{ fontSize: '12px', color: isDark ? '#a3a3a3' : '#64748b', marginTop: '2px' }}>
                                    {song.artist || '—'}
                                  </div>
                                </div>
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                  <Badge appearance="outline" color="informative" size="small">
                                    {song.defaultKey || song.arrangements?.[0]?.key || 'C'}
                                  </Badge>
                                  <span style={{ fontSize: '11px', color: isDark ? '#737373' : '#94a3b8' }}>
                                    {song.arrangements?.length || 1} {t('arrangements').toLowerCase()}
                                  </span>
                                </div>
                              </div>
                            ))
                          ) : (
                            <div style={{ padding: '24px', textAlign: 'center', color: isDark ? '#737373' : '#94a3b8', fontSize: '13px' }}>
                              {t('noResultsFound')}
                            </div>
                          )}
                        </div>
                      </div>
                    ) : (
                      /* Selected Song Configuration */
                      <>
                        <div style={{
                          padding: '12px 14px',
                          borderRadius: tokens.borderRadiusMedium,
                          backgroundColor: isDark ? '#262626' : '#f1f5f9',
                          border: `1px solid ${isDark ? '#383838' : '#cbd5e1'}`,
                          display: 'flex',
                          justifyContent: 'space-between',
                          alignItems: 'center'
                        }}>
                          <div>
                            <div style={{ fontWeight: '700', fontSize: '14px', color: isDark ? '#ffffff' : '#0f172a' }}>
                              {formData.title}
                            </div>
                            {formData.subtitle && (
                              <div style={{ fontSize: '12px', color: isDark ? '#a3a3a3' : '#64748b' }}>
                                {formData.subtitle}
                              </div>
                            )}
                          </div>
                          <Button 
                            size="small" 
                            appearance="subtle" 
                            icon={<DismissRegular fontSize={14} />} 
                            onClick={handleClearSelectedSong}
                          >
                            {t('change')}
                          </Button>
                        </div>

                        {/* Arrangement Selector */}
                        {(() => {
                          const selectedSong = songs.find(s => s.id === formData.selectedSongId);
                          const arrangements = selectedSong?.arrangements || [];
                          if (arrangements.length > 1) {
                            return (
                              <Field label={t('arrangement')} required>
                                <Select
                                  value={formData.selectedArrangementId}
                                  onChange={(e) => {
                                    const arrId = e.target.value;
                                    const arr = arrangements.find(a => a.id === arrId);
                                    if (arr) {
                                      setFormData(prev => ({
                                        ...prev,
                                        selectedArrangementId: arr.id,
                                        arrangementName: arr.name,
                                        key: arr.key || prev.key,
                                        durationMins: Math.floor((arr.duration || 300) / 60),
                                        durationSecs: (arr.duration || 300) % 60
                                      }));
                                    }
                                  }}
                                >
                                  {arrangements.map(arr => (
                                    <option key={arr.id} value={arr.id}>
                                      {arr.name} ({arr.key || 'C'}) — {Math.floor((arr.duration || 0)/60)}:{(arr.duration%60 || 0).toString().padStart(2, '0')}
                                    </option>
                                  ))}
                                </Select>
                              </Field>
                            );
                          }
                          return null;
                        })()}

                        <Field label={t('mainVocalistLead')}>
                          <Input 
                            value={formData.leadVocalist} 
                            onChange={e => setFormData({ ...formData, leadVocalist: e.target.value })} 
                            placeholder="e.g. Sarah M., Pastor Dave" 
                          />
                        </Field>

                        <Field label={t('extraNotesTechnicalCues')}>
                          <Textarea 
                            value={formData.notes} 
                            onChange={(e, data) => setFormData({ ...formData, notes: data.value })} 
                            placeholder="e.g. Acoustic guitar intro only, soft pad in background, key change at bridge..." 
                            rows={2}
                          />
                        </Field>
                      </>
                    )}
                  </>
                ) : (
                  /* Non-Song Items: Video, Loop, Announcements, Conference, Countdown */
                  <>
                    <Field label={t('titleName')} required>
                      <Input 
                        value={formData.title} 
                        onChange={e => setFormData({ ...formData, title: e.target.value })} 
                        placeholder={newItemType === 'video' ? 'e.g. Easter Promo Video' : newItemType === 'loop' ? 'e.g. Pre-Service Music Loop' : newItemType === 'conference' ? 'e.g. Sermon: The Power of Grace' : 'e.g. Welcome & Offering'} 
                        required 
                      />
                    </Field>

                    {newItemType === 'conference' && (
                      <Field label={t('speaker')}>
                        <Input 
                          value={formData.leadVocalist} 
                          onChange={e => setFormData({ ...formData, leadVocalist: e.target.value })} 
                          placeholder="e.g. Armando Flores, Pr. Roberto González" 
                        />
                      </Field>
                    )}

                    <div style={{ display: 'flex', gap: '16px' }}>
                      <Field label={t('durationMinutes')} style={{ flex: 1 }}>
                        <Input 
                          type="number" 
                          min="0" 
                          value={formData.durationMins} 
                          onChange={e => setFormData({ ...formData, durationMins: e.target.value })} 
                        />
                      </Field>
                      <Field label={t('durationSeconds')} style={{ flex: 1 }}>
                        <Input 
                          type="number" 
                          min="0" 
                          max="59" 
                          value={formData.durationSecs} 
                          onChange={e => setFormData({ ...formData, durationSecs: e.target.value })} 
                        />
                      </Field>
                    </div>

                    <Field label={t('extraNotesTechnicalCues')}>
                      <Textarea 
                        value={formData.notes} 
                        onChange={(e, data) => setFormData({ ...formData, notes: data.value })} 
                        placeholder="e.g. Play video on main projector, mute wireless mics..." 
                        rows={2} 
                      />
                    </Field>
                  </>
                )}
              </DialogContent>
              <DialogActions>
                <DialogTrigger disableButtonEnhancement>
                  <Button appearance="secondary">{t('cancel')}</Button>
                </DialogTrigger>
                <Button 
                  type="submit" 
                  appearance="primary"
                  disabled={newItemType === 'song' && !formData.selectedSongId}
                >
                  {t('addToSheet')}
                </Button>
              </DialogActions>
            </DialogBody>
          </form>
        </DialogSurface>
      </Dialog>
      </div>
    </div>
  );
}
