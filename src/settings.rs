use embedded_can::Id;

use crate::memory::controller::{
    configuration::DataBits,
    fifo::{FifoNumber, PayloadSize, RetransmissionAttempts},
};

#[derive(Debug, Default)]
pub enum PLL {
    #[default]
    Off,
    On,
}

#[derive(Debug, Default)]
pub enum SysClkDivider {
    #[default]
    DivByOne,
    DivByTwo,
}

#[derive(Debug, Default)]
pub struct OscillatorConfiguration {
    pub pll: PLL,
    pub divider: SysClkDivider,
}

impl OscillatorConfiguration {
    pub fn new(pll: PLL, divider: SysClkDivider) -> Self {
        Self { pll, divider }
    }
}

#[derive(Debug, Default)]
pub struct IoConfiguration {
    pub enable_tx_standby_pin: bool,
    pub tx_can_open_drain: bool,
    pub start_of_frame_on_clko: bool,
    pub interrupt_pin_open_drain: bool,
}

impl IoConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tx_standby_pin(mut self, enable_tx_standby_pin: bool) -> Self {
        self.enable_tx_standby_pin = enable_tx_standby_pin;
        self
    }

    pub fn with_tx_can_open_drain(mut self, tx_can_open_drain: bool) -> Self {
        self.tx_can_open_drain = tx_can_open_drain;
        self
    }

    pub fn with_start_of_frame_on_clko(mut self, start_of_frame_on_clko: bool) -> Self {
        self.start_of_frame_on_clko = start_of_frame_on_clko;
        self
    }

    pub fn interrupt_pin_open_drain(mut self, interrupt_pin_open_drain: bool) -> Self {
        self.interrupt_pin_open_drain = interrupt_pin_open_drain;
        self
    }
}

#[derive(Debug)]
pub struct TxEventFifoConfiguration {
    pub fifo_size: u8,
    pub enable_timestamps: bool,
    pub enable_fifo_overflow_interrupt: bool,
    pub enable_fifo_full_interrupt: bool,
    pub enable_fifo_half_full_interrupt: bool,
    pub enable_fifo_not_empty_interrupt: bool,
}

impl TxEventFifoConfiguration {
    pub fn new(fifo_size: u8) -> Self {
        Self {
            fifo_size,
            enable_timestamps: false,
            enable_fifo_overflow_interrupt: false,
            enable_fifo_full_interrupt: false,
            enable_fifo_half_full_interrupt: false,
            enable_fifo_not_empty_interrupt: false,
        }
    }

    pub fn with_timestamps(mut self, enable_timestamps: bool) -> Self {
        self.enable_timestamps = enable_timestamps;
        self
    }

    pub fn with_fifo_overflow_interrupt(mut self, enable_fifo_overflow_interrupt: bool) -> Self {
        self.enable_fifo_overflow_interrupt = enable_fifo_overflow_interrupt;
        self
    }

    pub fn with_fifo_full_interrupt(mut self, enable_fifo_full_interrupt: bool) -> Self {
        self.enable_fifo_full_interrupt = enable_fifo_full_interrupt;
        self
    }

    pub fn with_fifo_half_full_interrupt(mut self, enable_fifo_half_full_interrupt: bool) -> Self {
        self.enable_fifo_half_full_interrupt = enable_fifo_half_full_interrupt;
        self
    }

    pub fn with_fifo_not_empty_interrupt(mut self, enable_fifo_not_empty_interrupt: bool) -> Self {
        self.enable_fifo_not_empty_interrupt = enable_fifo_not_empty_interrupt;
        self
    }
}

#[derive(Debug)]
pub struct TxQueueConfiguration {
    pub message_priority: u8,
    pub retransmission_attempts: RetransmissionAttempts,
    pub fifo_size: u8,
    pub payload_size: PayloadSize,
    pub enable_transmit_attempts_exhausted_interrupt: bool,
    pub enable_queue_empty_interrupt: bool,
    pub enable_queue_not_full_interrupt: bool,
}

impl TxQueueConfiguration {
    pub fn new(message_priority: u8, fifo_size: u8, payload_size: PayloadSize) -> Self {
        Self {
            message_priority,
            retransmission_attempts: RetransmissionAttempts::default(),
            fifo_size,
            payload_size,
            enable_transmit_attempts_exhausted_interrupt: false,
            enable_queue_empty_interrupt: false,
            enable_queue_not_full_interrupt: false,
        }
    }

    pub fn with_retransmission_attempts(
        mut self,
        retransmission_attempts: RetransmissionAttempts,
    ) -> Self {
        self.retransmission_attempts = retransmission_attempts;
        self
    }

    pub fn with_transmit_attempts_exhausted_interrupt(
        mut self,
        enable_transmit_attempts_exhausted_interrupt: bool,
    ) -> Self {
        self.enable_transmit_attempts_exhausted_interrupt =
            enable_transmit_attempts_exhausted_interrupt;
        self
    }

    pub fn with_queue_empty_interrupt(mut self, enable_queue_empty_interrupt: bool) -> Self {
        self.enable_queue_empty_interrupt = enable_queue_empty_interrupt;
        self
    }

    pub fn with_queue_not_full_interrupt(mut self, enable_queue_not_full_interrupt: bool) -> Self {
        self.enable_queue_not_full_interrupt = enable_queue_not_full_interrupt;
        self
    }
}

#[derive(Debug)]
pub struct Settings {
    pub oscillator: OscillatorConfiguration,
    pub io_configuration: IoConfiguration,
    pub tx_event_fifo: Option<TxEventFifoConfiguration>,
    pub tx_queue: Option<TxQueueConfiguration>,
    pub enable_time_based_counter: bool,
    pub data_bits_to_match: Option<DataBits>,
}

#[derive(Debug)]
pub enum FifoMode {
    Transmit(TxFifoConfiguration),
    Receive(RxFifoConfiguration),
}

#[derive(Debug)]
pub struct TxFifoConfiguration {
    /// See sfr::controller::fifo::HIGHEST_FIFO_PRIORITY
    pub priority: u8,
    pub retransmission_attempts: RetransmissionAttempts,
    pub enable_auto_rtr: bool,
    pub enable_transmit_attempts_exhausted_interrupt: bool,
    pub enable_fifo_empty_interrupt: bool,
    pub enable_fifo_half_empty_interrupt: bool,
    pub enable_fifo_not_full_interrupt: bool,
}

impl TxFifoConfiguration {
    pub fn new(priority: u8) -> Self {
        Self {
            priority,
            retransmission_attempts: RetransmissionAttempts::default(),
            enable_auto_rtr: false,
            enable_transmit_attempts_exhausted_interrupt: false,
            enable_fifo_empty_interrupt: false,
            enable_fifo_half_empty_interrupt: false,
            enable_fifo_not_full_interrupt: false,
        }
    }

    pub fn with_retransmission_attempts(
        mut self,
        retransmission_attempts: RetransmissionAttempts,
    ) -> Self {
        self.retransmission_attempts = retransmission_attempts;
        self
    }

    pub fn with_auto_rtr(mut self, enable_auto_rtr: bool) -> Self {
        self.enable_auto_rtr = enable_auto_rtr;
        self
    }

    pub fn with_transmit_attempts_exhausted_interrupt(
        mut self,
        enable_transmit_attempts_exhausted_interrupt: bool,
    ) -> Self {
        self.enable_transmit_attempts_exhausted_interrupt =
            enable_transmit_attempts_exhausted_interrupt;
        self
    }

    pub fn with_fifo_empty_interrupt(mut self, enable_fifo_empty_interrupt: bool) -> Self {
        self.enable_fifo_empty_interrupt = enable_fifo_empty_interrupt;
        self
    }

    pub fn with_fifo_half_empty_interrupt(
        mut self,
        enable_fifo_half_empty_interrupt: bool,
    ) -> Self {
        self.enable_fifo_half_empty_interrupt = enable_fifo_half_empty_interrupt;
        self
    }

    pub fn with_fifo_not_full_interrupt(mut self, enable_fifo_not_full_interrupt: bool) -> Self {
        self.enable_fifo_not_full_interrupt = enable_fifo_not_full_interrupt;
        self
    }
}

#[derive(Debug, Default)]
pub struct RxFifoConfiguration {
    pub enable_message_timestamps: bool,
    pub enable_fifo_overflow_interrupt: bool,
    pub enable_fifo_full_interrupt: bool,
    pub enable_fifo_half_full_interrupt: bool,
    pub enable_fifo_not_empty_interrupt: bool,
}

impl RxFifoConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message_timestamps(mut self, enable_message_timestamps: bool) -> Self {
        self.enable_message_timestamps = enable_message_timestamps;
        self
    }

    pub fn with_fifo_overflow_interrupt(mut self, enable_fifo_overflow_interrupt: bool) -> Self {
        self.enable_fifo_overflow_interrupt = enable_fifo_overflow_interrupt;
        self
    }

    pub fn with_fifo_full_interrupt(mut self, enable_fifo_full_interrupt: bool) -> Self {
        self.enable_fifo_full_interrupt = enable_fifo_full_interrupt;
        self
    }

    pub fn with_fifo_half_full_interrupt(mut self, enable_fifo_half_full_interrupt: bool) -> Self {
        self.enable_fifo_half_full_interrupt = enable_fifo_half_full_interrupt;
        self
    }

    pub fn with_fifo_not_empty_interrupt(mut self, enable_fifo_not_empty_interrupt: bool) -> Self {
        self.enable_fifo_not_empty_interrupt = enable_fifo_not_empty_interrupt;
        self
    }
}

#[derive(Debug)]
pub struct FifoConfiguration {
    /// Max number of messages that can be stored in this FIFO (0 to 32)
    pub fifo_size: u8,
    /// Max size for a payload in this FIFO
    pub payload_size: PayloadSize,
    /// Either TX or RX
    pub mode: FifoMode,
}

impl FifoConfiguration {
    pub fn new(fifo_size: u8, payload_size: PayloadSize, mode: FifoMode) -> Self {
        Self {
            fifo_size,
            payload_size,
            mode,
        }
    }
}

pub struct FilterConfiguration {
    pub buffer_pointer: FifoNumber,
    pub mode: FilterMatchMode,
    pub filter_bits: Id,
    pub mask_bits: Id,
}

pub enum FilterMatchMode {
    StandardOnly,
    ExtendedOnly,
    Both,
}
