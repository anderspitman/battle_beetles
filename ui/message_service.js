const messageServiceModule = (function() {

  class MessageService {
    constructor() {
      this.socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");
    }

    getSocket() {
      return this.socket;
    }

    selectBeetle({ beetleId }) {
      this.socket.send(JSON.stringify({
        message_type: 'select-beetle',
        beetle_id: beetleId,
        x: 0,
        y: 0,
      }))
    }

    selectedMoveCommand({ x, y }) {
      this.socket.send(JSON.stringify({
        message_type: 'selected-move-command',
        beetle_id: -1,
        x,
        y,
      }))
    }

    selectedInteractCommand({ beetleId }) {
      this.socket.send(JSON.stringify({
        message_type: 'selected-interact-command',
        beetle_id: beetleId,
        x: 0,
        y: 0,
      }))
    }

    deselectAllBeetles() {
      this.socket.send(JSON.stringify({
        message_type: 'deselect-all-beetles',
        beetle_id: -1,
        x: 0,
        y: 0,
      }))
    }
  }

  return {
    MessageService,
  }
}());
