import * as messages from './gen/messages_pb';

export class MessageService {
  constructor() {
    this.socket = new WebSocket("ws://127.0.0.1:4020", "battle-beetles");
  }

  getSocket() {
    return this.socket;
  }

  selectBeetle({ beetleId }) {
    const selectBeetleMessage = new messages.SelectBeetle();
    selectBeetleMessage.setBeetleId(beetleId);
    const uiMessage = new messages.UiMessage();
    uiMessage.setSelectBeetle(selectBeetleMessage);
    this.socket.send(uiMessage.serializeBinary());
  }

  selectedMoveCommand({ x, y }) {
    const message = new messages.SelectedMoveCommand();
    message.setX(x);
    message.setY(y);
    const uiMessage = new messages.UiMessage();
    uiMessage.setSelectedMoveCommand(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  createBeetle({ x, y }) {
    const message = new messages.CreateBeetle();
    message.setX(x);
    message.setY(y);
    const uiMessage = new messages.UiMessage();
    uiMessage.setCreateBeetle(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  selectedInteractCommand({ beetleId }) {
    const message = new messages.SelectedInteractCommand();
    message.setBeetleId(beetleId);
    const uiMessage = new messages.UiMessage();
    uiMessage.setSelectedInteractCommand(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  deselectAllBeetles() {
    const message = new messages.DeselectAllBeetles();
    const uiMessage = new messages.UiMessage();
    uiMessage.setDeselectAllBeetles(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  terminate() {
    const message = new messages.Terminate();
    const uiMessage = new messages.UiMessage();
    uiMessage.setTerminate(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  runSpeedSimulation() {
    const message = new messages.RunSpeedSimulation();
    const uiMessage = new messages.UiMessage();
    uiMessage.setRunSpeedSimulation(message);
    this.socket.send(uiMessage.serializeBinary());
  }

  runBattleSimulation() {
    const message = new messages.RunBattleSimulation();
    const uiMessage = new messages.UiMessage();
    uiMessage.setRunBattleSimulation(message);
    this.socket.send(uiMessage.serializeBinary());
  }
}
